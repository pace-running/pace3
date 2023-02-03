use crate::schema::rejected_transactions;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = rejected_transactions)]
pub struct NewRejectedTransaction<'a> {
    pub runner_ids: &'a str,
    pub date_of_payment: &'a str,
    pub reasons_for_payment: &'a str,
    pub payment_amount: &'a str,
    pub expected_amount: Option<&'a str>,
    pub currency: &'a str,
    pub payer_name: &'a str,
    pub iban: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct RejectedTransaction {
    pub runner_ids: String,
    pub date_of_payment: String,
    pub reasons_for_payment: String,
    pub payment_amount: String,
    pub expected_amount: Option<String>,
    pub currency: String,
    pub payer_name: String,
    pub iban: String,
}
