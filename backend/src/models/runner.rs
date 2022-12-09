use diesel::prelude::*;
use diesel::PgConnection;
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use serde::Serialize;

use crate::constants::{
    BLACKLIST_START_NUMBERS, CHARSET, REASON_FOR_PAYMENT_LENGTH, VERIFICATION_CODE_LENGTH,
};
use crate::get_next_start_number;
use crate::schema::runners;

use super::info::Info;

#[derive(Insertable)]
#[diesel(table_name = runners)]
pub struct NewRunner<'a> {
    pub start_number: i64,
    pub firstname: Option<&'a str>,
    pub lastname: Option<&'a str>,
    pub team: Option<&'a str>,
    pub email: Option<&'a str>,
    pub starting_point: &'a str,
    pub running_level: &'a str,
    pub donation: &'a str,
    pub reason_for_payment: &'a str,
    pub payment_status: &'a bool,
    pub verification_code: &'a str,
    pub tshirt_cost: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct Runner {
    pub id: i32,
    pub start_number: i64,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub team: Option<String>,
    pub email: Option<String>,
    pub starting_point: String,
    pub running_level: String,
    pub donation: String,
    pub reason_for_payment: String,
    pub payment_status: bool,
    pub verification_code: String,
    pub payment_confirmation_mail_sent: bool,
    pub tshirt_cost: String,
}

impl<'a> From<(&'a Info, i64, &'a str, &'a str, &'a str)> for NewRunner<'a> {
    fn from(
        info_with_start_number_and_payment: (&'a Info, i64, &'a str, &'a str, &'a str),
    ) -> Self {
        let (info, next_start_number, reason_for_payment, verification_code, tshirt_cost) =
            info_with_start_number_and_payment;

        NewRunner {
            start_number: next_start_number,
            firstname: Some(&info.runner_info.firstname),
            lastname: Some(&info.runner_info.lastname),
            team: Some(&info.runner_info.team),
            email: Some(&info.runner_info.email),
            starting_point: &info.runner_info.starting_point,
            running_level: &info.runner_info.running_level,
            donation: &info.runner_info.donation,
            reason_for_payment,
            payment_status: &false,
            verification_code,
            tshirt_cost,
        }
    }
}

pub fn next_start_number(conn: &mut PgConnection) -> i64 {
    let mut next = get_next_start_number(conn);

    while BLACKLIST_START_NUMBERS.contains(&next) {
        next = get_next_start_number(conn);
    }

    next
}

pub fn create_random_payment() -> String {
    let mut rng = rand::thread_rng();

    let reason_for_payment: String = (0..REASON_FOR_PAYMENT_LENGTH)
        .map(|_| {
            let index = rng.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect();

    format!("LGR-{}", reason_for_payment)
}

pub fn create_verification_code() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), VERIFICATION_CODE_LENGTH)
}

#[cfg(test)]
mod tests {
    use crate::builders::InfoBuilder;
    use crate::constants::VERIFICATION_CODE_LENGTH;
    use crate::establish_connection;

    use super::*;

    // For testing only
    fn restart_start_number(conn: &mut PgConnection) {
        use diesel::sql_query;

        sql_query("ALTER SEQUENCE runner_start_number_seq RESTART")
            .execute(conn)
            .expect("Error resetting start_number sequence");
    }

    #[actix_web::test]
    async fn unit_create_new_runner_test() {
        let info = InfoBuilder::minimal_default().build();
        let expected_start_number = 10;
        let expected_reason_for_payment = "LGR-HUMKD";
        let expected_verification_code =
            "8xGH3xrkTHeYBzBrgPO9YBSKO9rZGcc0e8MKjACQ0KuLrpjrwpy1MhehHHqNN1XX";
        let runner = NewRunner::from((
            &info,
            expected_start_number,
            expected_reason_for_payment,
            expected_verification_code,
            "0",
        ));

        assert_eq!(runner.firstname.unwrap(), info.runner_info.firstname);
        assert_eq!(runner.lastname.unwrap(), info.runner_info.lastname);
        assert_eq!(runner.team.unwrap(), info.runner_info.team);
        assert_eq!(runner.email.unwrap(), info.runner_info.email);
        assert_eq!(runner.starting_point, info.runner_info.starting_point);
        assert_eq!(runner.running_level, info.runner_info.running_level);
        assert_eq!(runner.donation, info.runner_info.donation);
        assert_eq!(runner.start_number, expected_start_number);
        assert_eq!(runner.reason_for_payment, expected_reason_for_payment);
        assert_eq!(runner.payment_status, &false);
        assert_eq!(runner.verification_code, expected_verification_code);
    }

    #[actix_web::test]
    async fn unit_reason_for_payment() {
        let reason_for_payment = create_random_payment();

        assert_eq!(reason_for_payment.len(), 9);
        assert!(reason_for_payment.as_str().contains("LGR-"));
        assert!(reason_for_payment
            .as_str()
            .to_uppercase()
            .eq(&reason_for_payment));
    }

    #[actix_web::test]
    async fn unit_create_verification_code() {
        let verification_code_1 = create_verification_code();
        let verification_code_2 = create_verification_code();

        assert_eq!(verification_code_1.len(), VERIFICATION_CODE_LENGTH);
        assert_eq!(verification_code_2.len(), VERIFICATION_CODE_LENGTH);
        assert_ne!(verification_code_1, verification_code_2)
    }

    #[test]
    fn integration_next_start_number_test_no_duplicates() {
        use std::collections::HashSet;

        let conn = &mut establish_connection();
        restart_start_number(conn);
        let mut generated = HashSet::new();

        for _ in 1..100 {
            let next = next_start_number(conn);
            assert!(!generated.contains(&next));
            generated.insert(next);
        }
    }

    #[test]
    fn integration_next_start_number_test_no_blacklisted() {
        let conn = &mut establish_connection();
        restart_start_number(conn);
        for _ in 1..100 {
            let next = next_start_number(conn);
            assert!(!BLACKLIST_START_NUMBERS.contains(&next));
        }
    }
}
