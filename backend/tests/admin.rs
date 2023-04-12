use pace::{
    get_connection_pool, insert_rejected_transaction,
    models::rejected_transaction::NewRejectedTransaction,
};

#[test]
fn put_rejected_transaction_into_database() {
    let conn = &mut get_connection_pool()
        .expect("Unable to get connection pool.")
        .get()
        .expect("Unable to get connection.");
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
