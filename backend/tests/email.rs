use pace::core::service::{EmailService, LettreTeraEmailService};
use pace::models::runner::{PaymentReference, Runner};
use std::sync::Arc;

pub mod helpers;
use crate::helpers::TestDatabase;
pub use helpers::TestEmailServer;
use pace::repository::PostgresThemeRepository;

#[test]
fn send_registration_confirmation_should_send_mail_containing_payment_reference_to_server() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let test_email_server = TestEmailServer::new(None).unwrap();

    let email_service = LettreTeraEmailService::new(
        test_email_server.get_configuration(),
        &helpers::TEMPLATES,
        Some("foo.com".to_string()),
        Arc::new(PostgresThemeRepository::new(pool.clone())),
    )
    .unwrap();

    let payment_reference = PaymentReference::random().to_string();
    let example_runner = Runner {
        id: 42,
        start_number: 9000,
        firstname: None,
        lastname: None,
        team: None,
        bsv_participant: false,
        email: Some("runner@whatever.com".to_string()),
        starting_point: String::new(),
        running_level: String::new(),
        donation: "10".to_string(),
        reason_for_payment: payment_reference.clone(),
        payment_status: false,
        verification_code: "verification-code".to_string(),
        payment_confirmation_mail_sent: false,
        tshirt_cost: "15".to_string(),
    };

    let result = email_service.send_registration_confirmation(example_runner);

    if result.is_err() {
        dbg!(result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());
    assert_eq!(
        test_email_server.get_last_sender_email_address(),
        Some("email@example.com".to_string())
    );
    assert_eq!(
        test_email_server.get_last_recipient_email_addresses(),
        vec!["runner@whatever.com".to_string()]
    );
    assert!(test_email_server
        .get_last_mail_data()
        .unwrap()
        .contains(&payment_reference));
}

#[test]
fn send_payment_confirmation_should_send_mail_containing_verification_code_to_server() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let test_email_server = TestEmailServer::new(None).unwrap();

    let email_service = LettreTeraEmailService::new(
        test_email_server.get_configuration(),
        &helpers::TEMPLATES,
        Some("foo.com".to_string()),
        Arc::new(PostgresThemeRepository::new(pool.clone())),
    )
    .unwrap();

    let verification_code = "a04da5a4-2932-42f9-8d1e-bf4455b68003";

    let example_runner = Runner {
        id: 42,
        start_number: 9000,
        firstname: None,
        lastname: None,
        team: None,
        bsv_participant: false,
        email: Some("runner@whatever.com".to_string()),
        starting_point: String::new(),
        running_level: String::new(),
        donation: "10".to_string(),
        reason_for_payment: PaymentReference::random().to_string(),
        payment_status: false,
        verification_code: verification_code.to_string(),
        payment_confirmation_mail_sent: false,
        tshirt_cost: "15".to_string(),
    };

    let result = email_service.send_payment_confirmation(example_runner);

    if result.is_err() {
        dbg!(result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());
    assert_eq!(
        test_email_server.get_last_sender_email_address(),
        Some("email@example.com".to_string())
    );
    assert_eq!(
        test_email_server.get_last_recipient_email_addresses(),
        vec!["runner@whatever.com".to_string()]
    );
    assert!(test_email_server
        .get_last_mail_data()
        .unwrap()
        .contains(&verification_code));
}

/* Try to send a larger number of emails concurrently
#[actix_web::test]
async fn send_multiple_emails_at_once() {
    use futures_util::stream::FuturesUnordered;
    use futures_util::StreamExt;

    let mut tasks = FuturesUnordered::new();

    let test_email_server = TestEmailServer::new(Some(2601)).unwrap();
    for i in 0..1300 {
        let email_configuration = test_email_server.get_configuration().clone();
        tasks.push(tokio::spawn(async move {
            let payment_reference = PaymentReference::random().to_string();
            let example_runner = Runner {
                id: i,
                start_number: 9000,
                firstname: None,
                lastname: None,
                team: None,
                bsv_participant: false,
                email: Some("runner@whatever.com".to_string()),
                starting_point: String::new(),
                running_level: String::new(),
                donation: "10".to_string(),
                reason_for_payment: payment_reference.clone(),
                payment_status: false,
                verification_code: "verification-code".to_string(),
                payment_confirmation_mail_sent: false,
                tshirt_cost: "15".to_string(),
            };

            let email_service = LettreTeraEmailService::new(
                email_configuration,
                &helpers::TEMPLATES,
                Some("foo.com".to_string()),
            )
            .unwrap();

            println!("sending mail to runner {}", i);
            email_service.send_registration_confirmation(example_runner)
        }));
    }

    while let Some(finished_task) = tasks.next().await {
        match finished_task {
            Ok(result) => {
                assert!(result.is_ok());
            }
            Err(e) => {
                dbg!(e);
            }
        };
    }
}
*/
