use crate::models::rejected_transaction::{NewRejectedTransaction, RejectedTransaction};
use crate::models::runner::{create_verification_code, Runner};
use crate::models::shipping::NewShipping;
use crate::models::users::{LoginData, LoginResponse, User};
use crate::services::email::send_payment_confirmation;
use crate::{
    establish_connection, insert_rejected_transaction, insert_shipping, is_eu_country,
    retrieve_donation_by_reason_for_payment, retrieve_runner_by_id, retrieve_shipping_by_runner_id,
};
use actix_identity::Identity;
use actix_web::http::StatusCode;
use actix_web::web::{self, Json};
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use chrono::NaiveDate;
use diesel::prelude::*;
use futures_util::stream::StreamExt as _;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct QueryInfo {
    page_number: i32,
    search_category: String,
    search_keyword: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct FullRunnerDetails {
    runner_id: String,
    firstname: String,
    lastname: String,
    team: String,
    email: String,
    starting_point: String,
    running_level: String,
    donation: String,
    start_number: String,
    verification_code: String,
    reason_for_payment: String,
    payment_status: bool,
    payment_confirmation_mail_sent: bool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct ShippingDetails {
    tshirt_model: String,
    tshirt_size: String,
    country: String,
    address_firstname: String,
    address_lastname: String,
    street_name: String,
    house_number: String,
    address_extra: Option<String>,
    postal_code: String,
    city: String,
    delivery_status: String,
}
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Response {
    success_message: Option<String>,
    error_message: Option<String>,
    status_code: u16,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct FullRunnerResponse {
    is_tshirt_booked: bool,
    #[serde(flatten)]
    runner_details: Option<FullRunnerDetails>,
    #[serde(flatten)]
    shipping_details: Option<ShippingDetails>,
    #[serde(flatten)]
    inner_response: Response,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct FullRunnerInfo {
    #[serde(flatten)]
    runner_details: Option<FullRunnerDetails>,
    is_tshirt_booked: bool,
    #[serde(flatten)]
    shipping_details: Option<ShippingDetails>,
}

// TODO: delete  after changing implementation of csv parsing
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct FaultyTransaction {
    runner_ids: Option<Vec<String>>,
    reason_for_payment: String,
    amount: String,
    expected_amount: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct RejectedTransactionsResponse {
    transaction_list: Vec<RejectedTransaction>,
    inner_response: Response,
}

#[derive(Deserialize, Serialize)]
pub struct RunnerListResponse {
    runner_list: Vec<Runner>,
    stats_number: usize,
    stats_hamburg: i32,
    stats_total_donation: i32,
}

pub async fn check_password(
    request: HttpRequest,
    login_data: Json<LoginData>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let database_result = users
        .filter(username.like(&login_data.username))
        .first::<User>(connection);
    let user = match database_result {
        Ok(user) => user,
        Err(_) => User::default(),
    };
    if user.eq(&login_data.into_inner()) {
        let response = LoginResponse::from(&user);
        let json = serde_json::to_string(&response)?;
        Identity::login(&request.extensions(), response.username).unwrap();
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(json))
    } else {
        Ok(forbidden())
    }
}

pub async fn show_runners(
    _: Identity,
    params: web::Query<QueryInfo>,
) -> Result<HttpResponse, Error> {
    use crate::schema::runners::dsl::*;
    let connection = &mut establish_connection();
    let rows_per_page = 15;
    let query_info = params.into_inner();
    let page_number = query_info.page_number;
    let search_cat = query_info.search_category;
    let search_keyword = query_info.search_keyword;

    let database_result = runners.load::<Runner>(connection).unwrap();
    let mut filtered_result = apply_search_filter(&database_result, search_cat, search_keyword);

    let stats_number = filtered_result.len();
    let mut stats_hamburg = 0;
    let mut stats_total_donation = 0;
    for r in &filtered_result {
        if r.starting_point == "hamburg" {
            stats_hamburg += 1;
        }
        stats_total_donation += r.donation.parse().unwrap_or(0);
    }
    let last_index = std::cmp::min(
        page_number * rows_per_page,
        stats_number.try_into().unwrap(),
    );
    filtered_result.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    let response = RunnerListResponse {
        runner_list: filtered_result
            [((page_number - 1) * rows_per_page) as usize..(last_index as usize)]
            .to_vec(),
        stats_number,
        stats_hamburg,
        stats_total_donation,
    };
    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body(serde_json::to_string(&response).unwrap()))
}

fn apply_search_filter(old_list: &Vec<Runner>, cat: String, key: String) -> Vec<Runner> {
    let mut list = old_list.to_owned();
    if cat == "name" {
        list.retain(|r| {
            let full_name = format!(
                "{} {}",
                r.firstname.clone().unwrap_or("".to_string()),
                r.lastname.clone().unwrap_or("".to_string())
            );
            full_name.contains(&key)
        });
    } else if cat == "start_number" {
        let key_number = key.parse::<i64>().unwrap_or(0);
        list.retain(|r| r.start_number == key_number);
    } else if cat == "email" {
        list.retain(|r| r.email.as_ref().unwrap().contains(&key));
    } else if cat == "reason_for_payment" {
        let uc_key = key.to_uppercase();
        list.retain(|r| r.reason_for_payment.contains(&uc_key));
    }
    list
}

pub async fn modify_payment_status(
    _: Identity,
    r_id: web::Path<i32>,
    target_status: Json<bool>,
) -> Result<HttpResponse, Error> {
    let runner_id = r_id.into_inner();
    let connection = &mut establish_connection();
    let result = change_payment_status(connection, runner_id, target_status.into_inner());
    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body(serde_json::to_string(&result).unwrap()))
}

pub async fn get_full_runner(
    _: Identity,
    request_data: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let runner_id = request_data.into_inner();
    let connection = &mut establish_connection();

    let retrieved_runner = retrieve_runner_by_id(connection, runner_id);
    let retrieved_shipping_result = retrieve_shipping_by_runner_id(connection, runner_id);

    let inner_response = Response {
        success_message: Some("Data received".to_string()),
        error_message: None,
        status_code: StatusCode::OK.as_u16(),
    };

    let runner_details = Option::from(FullRunnerDetails {
        runner_id: retrieved_runner.id.to_string(),
        firstname: retrieved_runner.firstname.unwrap_or_default(),
        lastname: retrieved_runner.lastname.unwrap_or_default(),
        team: retrieved_runner.team.unwrap_or_default(),
        email: retrieved_runner.email.unwrap_or_default(),
        starting_point: retrieved_runner.starting_point,
        running_level: retrieved_runner.running_level,
        donation: retrieved_runner.donation,
        start_number: retrieved_runner.start_number.to_string(),
        verification_code: retrieved_runner.verification_code,
        reason_for_payment: retrieved_runner.reason_for_payment,
        payment_status: retrieved_runner.payment_status,
        payment_confirmation_mail_sent: retrieved_runner.payment_confirmation_mail_sent,
    });

    match retrieved_shipping_result {
        Ok(shipping) => Ok(HttpResponse::Ok().json(FullRunnerResponse {
            runner_details,
            is_tshirt_booked: true,
            shipping_details: Option::from(ShippingDetails {
                tshirt_model: shipping.tshirt_model,
                tshirt_size: shipping.tshirt_size,
                country: shipping.country,
                address_firstname: shipping.firstname,
                address_lastname: shipping.lastname,
                street_name: shipping.street_name,
                house_number: shipping.house_number,
                address_extra: shipping.address_extra,
                postal_code: shipping.postal_code,
                city: shipping.city,
                delivery_status: shipping.delivery_status,
            }),
            inner_response,
        })),
        Err(_) => Ok(HttpResponse::Ok().json(FullRunnerResponse {
            runner_details,
            is_tshirt_booked: false,
            shipping_details: None,
            inner_response,
        })),
    }
}

pub async fn edit_runner(
    _: Identity,
    request_data: web::Path<i32>,
    form: Json<FullRunnerInfo>,
) -> Result<HttpResponse, Error> {
    #[allow(non_snake_case)] // not snake case to avoid confusion with shippings column
    let runner_ID = request_data.into_inner();
    let info = form.into_inner();
    use crate::schema::runners::dsl::*;

    let connection = &mut establish_connection();
    // println!("runner_id: {}", runner_ID);
    // println!("info: {:?}", info);

    let runner_details = info.runner_details.unwrap();
    // calculate new tshirt cost
    let shipping_details = info.shipping_details.unwrap();
    let new_tshirt_cost;
    if info.is_tshirt_booked {
        if shipping_details.country == "Deutschland" {
            new_tshirt_cost = "15";
        } else if is_eu_country(&shipping_details.country) {
            new_tshirt_cost = "17";
        } else {
            new_tshirt_cost = "20";
        }
    } else {
        new_tshirt_cost = "0";
    }

    // change runner
    let updated_runner = diesel::update(runners.find(runner_ID))
        .set((
            start_number.eq(runner_details.start_number.parse::<i64>().unwrap_or(-1)),
            firstname.eq(runner_details.firstname),
            lastname.eq(runner_details.lastname),
            team.eq(runner_details.team),
            email.eq(runner_details.email),
            starting_point.eq(runner_details.starting_point),
            running_level.eq(runner_details.running_level),
            donation.eq(runner_details.donation),
            reason_for_payment.eq(runner_details.reason_for_payment),
            payment_status.eq(runner_details.payment_status),
            verification_code.eq(runner_details.verification_code),
            tshirt_cost.eq(new_tshirt_cost),
        ))
        .get_result::<Runner>(connection)
        .unwrap();

    // delete old shipping, then insert new one
    if info.is_tshirt_booked {
        use crate::schema::shippings::dsl::*;
        let _ = diesel::delete(shippings.filter(runner_id.eq(runner_ID))).execute(connection);

        insert_shipping(
            connection,
            NewShipping {
                tshirt_model: &shipping_details.tshirt_model,
                tshirt_size: &shipping_details.tshirt_size,
                country: &shipping_details.country,
                firstname: &shipping_details.address_firstname,
                lastname: &shipping_details.address_lastname,
                street_name: &shipping_details.street_name,
                house_number: &shipping_details.house_number,
                address_extra: Some(&shipping_details.address_extra.unwrap_or_default()),
                postal_code: &shipping_details.postal_code,
                city: &shipping_details.city,
                runner_id: runner_ID,
                delivery_status: &shipping_details.delivery_status,
            },
        );
    } else {
        use crate::schema::shippings::dsl::*;
        let _ = diesel::delete(shippings.filter(runner_id.eq(runner_ID))).execute(connection);
    }

    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body(serde_json::to_string(&updated_runner).unwrap()))
}

pub async fn parse_payment_csv(
    _: Identity,
    mut raw_data: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut bytes_mut = web::BytesMut::new();
    while let Some(item) = raw_data.next().await {
        bytes_mut.extend_from_slice(&item?);
    }
    // println!("Bytes: {:?}", bytes_mut);
    let bytes = &bytes_mut.freeze();
    let csv_string;
    unsafe {
        csv_string = std::str::from_utf8_unchecked(bytes);
    }
    // println!("String: {}",csv_string);
    let mut reader = csv::Reader::from_reader(csv_string.as_bytes());

    let mut accepted = 0;
    let mut rejected = 0;
    for record in reader.byte_records() {
        let record = record.unwrap_or_default();
        let record_response = register_payment(&String::from_utf8_lossy(record.as_slice()));
        if record_response == "accepted" {
            accepted += 1;
        } else if record_response == "rejected" {
            rejected += 1;
        }
    }
    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body(serde_json::to_string(&[accepted, rejected]).unwrap()))
}

fn register_payment(row: &str) -> String {
    let entries = row.split(";").collect::<Vec<_>>();

    let connection = &mut establish_connection();

    let rfp_string = entries[9];
    let rfp_list = filter_rfp(rfp_string);
    // println!("{:?}", rfp_list);
    if rfp_list.len() == 0 {
        return "empty".to_string();
    }
    let paid_amount = entries[12];
    let mut budget: i32 = paid_amount.trim().parse().unwrap_or(0);

    let mut successful_ids: Vec<String> = Vec::new();
    let mut new_transaction = NewRejectedTransaction {
        runner_ids: "",
        date_of_payment: entries[0],
        reasons_for_payment: &rfp_list.join(", "),
        payment_amount: &paid_amount,
        expected_amount: None,
        currency: entries[11],
        payer_name: entries[4],
        iban: entries[6],
    };

    for rfp in &rfp_list {
        let result = retrieve_donation_by_reason_for_payment(connection, &rfp);
        match result {
            Err(_) => {
                let runner_ids = &successful_ids.join(", ");
                new_transaction.runner_ids = runner_ids;
                insert_rejected_transaction(connection, new_transaction);
                return "rejected".to_string();
            }
            Ok(returned_runner) => {
                successful_ids.push(returned_runner.id.to_string());
                budget = budget
                    - returned_runner.donation.trim().parse::<i32>().unwrap()
                    - returned_runner.tshirt_cost.trim().parse::<i32>().unwrap();
            }
        }
    }
    if budget >= 0 {
        for id in successful_ids {
            change_payment_status(connection, id.trim().parse::<i32>().unwrap(), true);
        }
        return "accepted".to_string();
    } else {
        let runner_ids = &successful_ids.join(", ");
        new_transaction.runner_ids = runner_ids;
        let expected = (paid_amount.trim().parse().unwrap_or(0) - budget).to_string();
        new_transaction.expected_amount = Some(&expected);
        insert_rejected_transaction(connection, new_transaction);
        return "rejected".to_string();
    }
}

fn filter_rfp(rfp: &str) -> Vec<String> {
    let mut list: Vec<String> = vec![];
    let char_array: Vec<char> = rfp.chars().collect();
    if char_array.len() <= 9 {
        return list;
    }
    for i in 0..(char_array.len() - 4) {
        if char_array[i] == 'L' && i < char_array.len() - 8 {
            if &char_array[i + 1..i + 4].into_iter().collect::<String>() == "GR-" {
                // println!("suffix: {:?}", &char_array[i+4..i+9]);
                let mut reason = char_array[i..i + 9]
                    .into_iter()
                    .collect::<String>()
                    .to_uppercase();
                reason = str::replace(&reason, "0", "O").to_string();
                list.push(reason);
            }
        }
    }
    list
}

pub async fn logout(user: Identity) -> Result<HttpResponse, Error> {
    user.logout();
    Ok(HttpResponse::NoContent().finish())
}

fn forbidden() -> HttpResponse {
    HttpResponse::Forbidden()
        .content_type("application/json")
        .body("\"result\": \"fail\"")
}

pub fn change_payment_status(
    conn: &mut PgConnection,
    runner_id: i32,
    target_status: bool,
) -> Runner {
    use crate::schema::runners::dsl::*;

    let result = diesel::update(runners.find(runner_id))
        .set(payment_status.eq(target_status))
        .get_result::<Runner>(conn)
        .unwrap();
    let email_value = result.email.as_ref().unwrap();
    let is_email_provided = email_value.ne("");
    let is_paid = result.payment_status;
    let mail_not_sent_yet = !result.payment_confirmation_mail_sent;
    if is_paid && is_email_provided && mail_not_sent_yet {
        send_payment_confirmation_email(&result);
        let _ = diesel::update(runners.find(runner_id))
            .set(payment_confirmation_mail_sent.eq(true))
            .get_result::<Runner>(conn);
    }
    result
}

fn send_payment_confirmation_email(runner: &Runner) -> bool {
    let email_value = runner.email.as_ref().unwrap();
    let verification_code = create_verification_code();
    send_payment_confirmation(
        runner.id.to_string(),
        runner.start_number.to_string(),
        email_value.to_string(),
        runner.donation.to_string(),
        verification_code,
        runner.tshirt_cost.to_string(),
    )
}

pub async fn get_rejected_transactions(_: Identity) -> Result<HttpResponse, Error> {
    use crate::schema::rejected_transactions::dsl::*;
    let connection = &mut establish_connection();
    let mut transaction_list = rejected_transactions
        .load::<RejectedTransaction>(connection)
        .unwrap();
    transaction_list.sort_by(|a, b| {
        NaiveDate::parse_from_str(&b.date_of_payment, "%d.%m.%Y")
            .unwrap_or_default()
            .partial_cmp(
                &NaiveDate::parse_from_str(&a.date_of_payment, "%d.%m.%Y").unwrap_or_default(),
            )
            .unwrap()
    });

    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body(serde_json::to_string(&transaction_list).unwrap()))
}

#[cfg(test)]
mod tests {
    use diesel::result::Error;
    use diesel::{Connection, RunQueryDsl};
    use crate::{establish_connection, insert_rejected_transaction, models::rejected_transaction::NewRejectedTransaction, schema};
    use crate::schema::users::{role, username};

    use super::filter_rfp;

    #[test]
    fn unit_reason_for_payment_is_extracted_from_string() {
        let rfp = "Vwz: �berweisung LGR-TTZLK und LGR-we0gS";
        let result = filter_rfp(rfp);
        assert_eq!(result, ["LGR-TTZLK", "LGR-WEOGS"]);
    }

    #[test]
    fn integration_put_rej_trans_into_database() {
        let conn = &mut establish_connection();
        let new_transaction = NewRejectedTransaction {
            runner_ids: "2, 5",
            date_of_payment: "03.02.2023",
            reasons_for_payment: "LGR-POIUY, LGR-QWERT",
            payment_amount: "44",
            expected_amount: Some("45"),
            currency: "EUR",
            payer_name: "Testy McTest",
            iban: "DE87876876876",
        };
        let inserted_transaction = insert_rejected_transaction(conn, new_transaction);
        assert_eq!(inserted_transaction.iban, "DE87876876876");
    }

    #[test]
    fn unit_test_test_connection() {
        let conn = &mut establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(schema::users::table)
                .values(username.eq("testuser"), role.eq("nonadmin"))
                .execute(conn)?;
            let all_names = schema::users.select(username, role).load::<String>(&conn)?;
            assert_eq!(vec!["Sean", "Tess", "Ruby"], all_names);
            Ok(())
        });
    }
}
