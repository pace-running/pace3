use crate::schema::rejected_transactions;
use crate::DbPool;
use actix_web::web;
use chrono::NaiveDateTime;
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
    // pub entry_added_at: &'a DateTime<Utc>,
}

#[derive(Queryable, Clone)]
pub struct RejectedTransaction {
    pub id: i32,
    pub runner_ids: String,
    pub date_of_payment: String,
    pub reasons_for_payment: String,
    pub payment_amount: String,
    pub expected_amount: Option<String>,
    pub currency: String,
    pub payer_name: String,
    pub iban: String,
    pub entry_added_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone)]

pub struct RejectedTransactionWithPotentialDuplicates {
    pub id: i32,
    pub runner_ids: String,
    pub date_of_payment: String,
    pub reasons_for_payment: String,
    pub payment_amount: String,
    pub expected_amount: Option<String>,
    pub currency: String,
    pub payer_name: String,
    pub iban: String,
    pub entry_added_at: String,
    pub possible_duplicate: bool,
}

pub fn find_duplicates(
    transaction: RejectedTransaction,
    db_pool: web::Data<DbPool>,
) -> RejectedTransactionWithPotentialDuplicates {
    use crate::schema::rejected_transactions::dsl::*;
    let connection = &mut db_pool.get().unwrap();
    let transaction_list = rejected_transactions
        .load::<RejectedTransaction>(connection)
        .unwrap();
    let mut duplicate_found = false;

    for found_transaction in transaction_list {
        if transaction.id == found_transaction.id
            && transaction.runner_ids == found_transaction.runner_ids
            && transaction.date_of_payment == found_transaction.date_of_payment
            && transaction.reasons_for_payment == found_transaction.reasons_for_payment
            && transaction.payment_amount == found_transaction.payment_amount
            && transaction.currency == found_transaction.currency
            && transaction.payer_name == found_transaction.payer_name
            && transaction.iban == found_transaction.iban
            && transaction.entry_added_at > found_transaction.entry_added_at
        {
            duplicate_found = true;
            break;
        }
    }

    RejectedTransactionWithPotentialDuplicates {
        id: transaction.id,
        runner_ids: transaction.runner_ids,
        date_of_payment: transaction.date_of_payment,
        reasons_for_payment: transaction.reasons_for_payment,
        payment_amount: transaction.payment_amount,
        expected_amount: transaction.expected_amount,
        currency: transaction.currency,
        payer_name: transaction.payer_name,
        iban: transaction.iban,
        entry_added_at: transaction.entry_added_at.to_string(),
        possible_duplicate: duplicate_found,
    }
}
