use crate::core::service::{
    EmailService, PageParameters, PaymentService, PaymentStatus, PaymentUpdateError,
    RunnerSearchFilter, RunnerSearchParameters, RunnerService, UserService,
};
use crate::models::rejected_transaction::{
    find_duplicates, NewRejectedTransaction, RejectedTransaction,
};
use crate::models::runner::Runner;
use crate::models::shipping::NewShipping;
use crate::models::users::{LoginData, LoginResponse, PasswordChangeData};
use crate::{
    handlers, insert_rejected_transaction, insert_shipping, is_eu_country,
    retrieve_donation_by_reason_for_payment, retrieve_shipping_by_runner_id, DbPool,
};
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::http::{header, StatusCode};
use actix_web::web;
use actix_web::{error, Error, HttpMessage, HttpRequest, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use futures_util::stream::StreamExt as _;
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::{Date, Month};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct QueryInfo {
    page_number: i32,
    search_category: String,
    search_keyword: String,
    show_only_bsv: bool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct FullRunnerDetails {
    runner_id: String,
    firstname: String,
    lastname: String,
    team: String,
    bsv_participant: bool,
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

#[derive(Deserialize, Serialize)]
pub struct RunnerListResponse {
    runner_list: Vec<Runner>,
    stats_number: usize,
    stats_hamburg: i32,
    stats_total_donation: i32,
}

pub async fn login(
    request: HttpRequest,
    login_data: web::Json<LoginData>,
    user_service: web::Data<dyn UserService>,
) -> Result<HttpResponse, Error> {
    let user = user_service
        .find_user_by_username(login_data.username.to_string())
        .ok_or(handlers::error::ClientError::AuthorizationError)?;

    if user.eq(&login_data.into_inner()) {
        let response = LoginResponse::from(&user);
        let json = serde_json::to_string(&response)?;
        Identity::login(&request.extensions(), response.username).unwrap();
        Ok(HttpResponse::Ok()
            .content_type(header::ContentType::json())
            .body(json))
    } else {
        Err(Error::from(
            handlers::error::ClientError::AuthorizationError,
        ))
    }
}

pub async fn change_password(
    current_user: Identity,
    change_password_data: web::Json<PasswordChangeData>, // talisman-ignore-line
    user_service: web::Data<dyn UserService>,
) -> Result<HttpResponse, Error> {
    let username = current_user.id().unwrap();

    if change_password_data.old_password.is_empty() || change_password_data.new_password.is_empty()
    {
        return Err(Error::from(
            handlers::error::ClientError::AuthorizationError,
        ));
    }
    let user = user_service
        .find_user_by_username(username.to_string())
        .ok_or(handlers::error::ClientError::AuthorizationError)?;
    let login_data = LoginData {
        username: username.clone(),
        password: change_password_data.old_password.to_string(), // talisman-ignore-line
    };
    if user.eq(&login_data) {
        user_service
            .change_password(
                username,
                change_password_data.old_password.to_string(),
                change_password_data.new_password.to_string(),
            )
            .map_err(handlers::error::InternalError::from)?;
        let response = LoginResponse::from(&user);
        let json = serde_json::to_string(&response)?;
        Ok(HttpResponse::Ok()
            .content_type(header::ContentType::json())
            .body(json))
    } else {
        Err(Error::from(
            handlers::error::ClientError::AuthorizationError,
        ))
    }
}

pub async fn show_runners(
    _: Identity,
    params: web::Query<QueryInfo>,
    runner_service: web::Data<dyn RunnerService>,
) -> Result<HttpResponse, Error> {
    let query_info = params.into_inner();
    let page_number = query_info.page_number - 1;
    let search_cat = query_info.search_category;
    let search_keyword = query_info.search_keyword;
    let show_only_bsv = query_info.show_only_bsv;

    let search_filter = RunnerSearchFilter::from_category_and_keyword(&search_cat, &search_keyword) // talisman-ignore-line
        .map_err(|_| handlers::error::ClientError::BadRequestError)?;
    let bsv_participant_filter = if show_only_bsv { Some(true) } else { None };
    let page_parameters = PageParameters::try_from(page_number)
        .map_err(|_| handlers::error::ClientError::BadRequestError)?;
    let search_result = runner_service.find_runners_by_search_parameters(
        RunnerSearchParameters::new(search_filter, bsv_participant_filter, page_parameters),
    );

    let response_body = RunnerListResponse {
        runner_list: search_result.runners,
        stats_number: search_result.stats.count_total_results,
        stats_hamburg: search_result.stats.count_starting_point as i32,
        stats_total_donation: search_result.stats.count_donations,
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&response_body)?))
}

pub async fn modify_payment_status(
    _: Identity,
    runner_id: web::Path<i32>,
    target_status: web::Json<bool>,
    payment_service: web::Data<dyn PaymentService>,
) -> Result<HttpResponse, Error> {
    let payment_status = if target_status.into_inner() {
        PaymentStatus::Paid
    } else {
        PaymentStatus::Pending
    };

    let runner = payment_service
        .set_payment_status(runner_id.into_inner(), payment_status)
        .map_err(|e| match e {
            PaymentUpdateError::UserNotFound => {
                Error::from(handlers::error::ClientError::BadRequestError)
            }
            PaymentUpdateError::UnableToSendEmail => {
                Error::from(handlers::error::InternalError::from(e))
            }
        })?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&runner).unwrap()))
}

pub async fn get_full_runner(
    _: Identity,
    request_data: web::Path<i32>,
    db_pool: web::Data<DbPool>,
    runner_service: web::Data<dyn RunnerService>,
) -> Result<HttpResponse, Error> {
    let runner_id = request_data.into_inner();
    let connection = &mut db_pool.get().map_err(error::ErrorInternalServerError)?;

    let retrieved_runner = runner_service
        .find_runner_by_id(runner_id)
        .unwrap_or_else(|| panic!("Unable to find runner with id {runner_id}."));
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
        bsv_participant: retrieved_runner.bsv_participant,
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
    form: web::Json<FullRunnerInfo>,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    #[allow(non_snake_case)] // not snake case to avoid confusion with shippings column
    let runner_ID = request_data.into_inner();
    let info = form.into_inner();
    use crate::schema::runners::dsl::*;

    let connection = &mut db_pool.get().map_err(error::ErrorInternalServerError)?;
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
            bsv_participant.eq(runner_details.bsv_participant),
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
    db_pool: web::Data<DbPool>,
    email_service: web::Data<dyn EmailService>,
) -> Result<HttpResponse, Error> {
    let connection = &mut db_pool.get().map_err(error::ErrorInternalServerError)?;
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
        let record_response = register_payment(
            &String::from_utf8_lossy(record.as_slice()),
            connection,
            &email_service,
        );
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

fn register_payment(
    row: &str,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    email_service: &web::Data<dyn EmailService>,
) -> String {
    let entries = row.split(';').collect::<Vec<_>>();

    let rfp_string = entries[9];
    let rfp_list = filter_rfp(rfp_string);
    // println!("{:?}", rfp_list);
    if rfp_list.is_empty() {
        return "empty".to_string();
    }
    let paid_amount = entries[12];
    let mut budget: i32 = paid_amount.trim().parse().unwrap_or(0);

    let mut successful_ids: Vec<String> = Vec::new();
    let mut new_transaction = NewRejectedTransaction {
        runner_ids: "",
        date_of_payment: entries[0],
        reasons_for_payment: &rfp_list.join(", "),
        payment_amount: paid_amount,
        expected_amount: None,
        currency: entries[11],
        payer_name: entries[4],
        iban: entries[6],
    };

    for rfp in &rfp_list {
        let result = retrieve_donation_by_reason_for_payment(connection, rfp);
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
            change_payment_status(
                connection,
                id.trim().parse::<i32>().unwrap(),
                true,
                email_service,
            );
        }
        "accepted".to_string()
    } else {
        let runner_ids = &successful_ids.join(", ");
        new_transaction.runner_ids = runner_ids;
        let expected = (paid_amount.trim().parse().unwrap_or(0) - budget).to_string();
        new_transaction.expected_amount = Some(&expected);
        insert_rejected_transaction(connection, new_transaction);
        "rejected".to_string()
    }
}

fn filter_rfp(rfp: &str) -> Vec<String> {
    let mut list: Vec<String> = vec![];
    let char_array: Vec<char> = rfp.chars().collect();
    if char_array.len() <= 9 {
        return list;
    }
    for i in 0..(char_array.len() - 4) {
        if char_array[i] == 'L'
            && i < char_array.len() - 8
            && &char_array[i + 1..i + 4].iter().collect::<String>() == "GR-"
        {
            let mut reason = char_array[i..i + 9]
                .iter()
                .collect::<String>()
                .to_uppercase();
            reason = str::replace(&reason, "0", "O").to_string();
            list.push(reason);
        }
    }
    list
}

pub async fn logout(user: Identity) -> Result<HttpResponse, Error> {
    user.logout();
    Ok(HttpResponse::NoContent().finish())
}

pub fn change_payment_status(
    conn: &mut PgConnection,
    runner_id: i32,
    target_status: bool,
    email_service: &web::Data<dyn EmailService>,
) -> Runner {
    use crate::schema::runners::dsl::*;

    let runner = diesel::update(runners.find(runner_id))
        .set(payment_status.eq(target_status))
        .get_result::<Runner>(conn)
        .unwrap();
    let is_email_provided = runner.email.is_some();
    let is_paid = runner.payment_status;
    let mail_not_sent_yet = !runner.payment_confirmation_mail_sent;
    if is_paid && is_email_provided && mail_not_sent_yet {
        let _email_result = email_service.send_payment_confirmation(runner.clone());
        let _ = diesel::update(runners.find(runner_id))
            .set(payment_confirmation_mail_sent.eq(true))
            .get_result::<Runner>(conn);
    }
    runner
}

pub async fn get_rejected_transactions(
    _: Identity,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    use crate::schema::rejected_transactions::dsl::*;
    let connection = &mut db_pool.get().map_err(error::ErrorInternalServerError)?;
    let mut transaction_list = rejected_transactions
        .load::<RejectedTransaction>(connection)
        .unwrap();
    transaction_list.sort_by(|a, b| {
        Date::parse(
            &b.date_of_payment,
            &time::format_description::well_known::Rfc2822,
        )
        .unwrap_or_else(|_| Date::from_calendar_date(1970, Month::January, 1).unwrap())
        .partial_cmp(
            &Date::parse(
                &a.date_of_payment,
                &time::format_description::well_known::Rfc2822,
            )
            .unwrap_or_else(|_| Date::from_calendar_date(1970, Month::January, 1).unwrap()),
        )
        .unwrap()
    });

    let mut new_transaction_list = Vec::new();
    for transaction in transaction_list {
        new_transaction_list.push(find_duplicates(transaction, db_pool.clone()));
    }

    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body(serde_json::to_string(&new_transaction_list).unwrap()))
}

pub async fn delete_rejected_transactions(
    _: Identity,
    ids: web::Json<Vec<i32>>,
    payment_service: web::Data<dyn PaymentService>,
) -> anyhow::Result<HttpResponse, Error> {
    let id_list = ids.into_inner();

    let number_of_deleted_rejected_transactions =
        payment_service.delete_rejected_transactions(id_list);

    let response_body_values = HashMap::from([(
        "deletedRejectedTransactions",
        number_of_deleted_rejected_transactions,
    )]);
    Ok(HttpResponse::Ok().json(response_body_values))
}

#[cfg(test)]
mod tests {
    use super::filter_rfp;

    #[actix_web::test]
    async fn unit_reason_for_payment_is_extracted_from_string() {
        let rfp = "Vwz: �berweisung LGR-TTZLK und LGR-we0gS";
        let result = filter_rfp(rfp);
        assert_eq!(result, ["LGR-TTZLK", "LGR-WEOGS"]);
    }
}
