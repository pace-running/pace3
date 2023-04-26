use pace::core::service::{EmailService, LettreTeraEmailService};
use pace::models::runner::{PaymentReference, Runner};

pub mod helpers;
pub use helpers::TestEmailServer;

#[test]
fn send_registration_confirmation_should_send_mail_containing_payment_reference_to_server() {
    let test_email_server = TestEmailServer::new().unwrap();

    let email_service = LettreTeraEmailService::new(
        test_email_server.get_configuration(),
        &helpers::TEMPLATES,
        Some("foo.com".to_string()),
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
        email: Some("foo@example.com".to_string()),
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
        test_email_server.get_last_mail_address(),
        Some("email@example.com".to_string())
    );
    assert!(test_email_server
        .get_last_mail_data()
        .unwrap()
        .contains(&payment_reference))
}

#[test]
fn send_payment_confirmation_should_send_mail_containing_verification_code_to_server() {
    let test_email_server = TestEmailServer::new().unwrap();

    let email_service = LettreTeraEmailService::new(
        test_email_server.get_configuration(),
        &helpers::TEMPLATES,
        Some("foo.com".to_string()),
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
        test_email_server.get_last_mail_address(),
        Some("email@example.com".to_string())
    );
    assert!(test_email_server
        .get_last_mail_data()
        .unwrap()
        .contains(&verification_code));
}
