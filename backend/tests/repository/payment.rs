use diesel::RunQueryDsl;
use pace::core::repository::PaymentRepository;
use pace::repository::PostgresPaymentRepository;

pub use crate::helpers::TestDatabase;

#[test]
fn deleted_rejected_transactions_by_ids_should_delete_all_relevant_entries() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let payment_repository = PostgresPaymentRepository::new(pool.clone());

    let mut connection = pool.get().unwrap();
    let _ = diesel::sql_query(
        r#"
INSERT INTO rejected_transactions (runner_ids, date_of_payment,
                                   reasons_for_payment, payment_amount,
                                   expected_amount, currency, payer_name, iban)
VALUES ('73', '2023-01-01', 'LGR-RANDO', '1', '20', 'EUR', 'Peter',
        'DE20 1342 6474 521 45'),
       ('42', '2023-01-01', 'LGR-EMPTY', '2', '10', 'EUR', 'Petra',
        'DE20 2342 5474 523 11');"#,
    )
    .execute(&mut connection)
    .unwrap();

    let result = payment_repository.delete_rejected_transactions_by_ids(vec![1, 2]);

    assert_eq!(2, result)
}

#[test]
fn deleted_rejected_transactions_by_ids_should_ignore_irrelevant_entries() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let payment_repository = PostgresPaymentRepository::new(pool.clone());

    let mut connection = pool.get().unwrap();
    let _ = diesel::sql_query(
        r#"
INSERT INTO rejected_transactions (runner_ids, date_of_payment,
                                   reasons_for_payment, payment_amount,
                                   expected_amount, currency, payer_name, iban)
VALUES ('73', '2023-01-01', 'LGR-PETER', '1', '20', 'EUR', 'Peter',
        'DE20 1342 6474 521 45'),
       ('42', '2023-01-01', 'LGR-PETRA', '2', '10', 'EUR', 'Petra',
        'DE20 2342 5474 523 11'),
       ('24', '2023-01-01', 'LGR-HANSI', '2', '10', 'EUR', 'Hansi',
        'DE20 1234 5474 523 34'),
       ('36', '2023-01-01', 'LGR-KARLA', '2', '10', 'EUR', 'Karla',
        'DE20 7743 5474 523 64');"#,
    )
    .execute(&mut connection)
    .unwrap();

    let result = payment_repository.delete_rejected_transactions_by_ids(vec![3, 4]);

    assert_eq!(2, result)
}
