use actix_web::web;
use dotenvy::dotenv;
use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Deserialize, Serialize)]
struct EmailDetails {
    receiver_email: String,
    template_name: &'static str,
    subject: &'static str,
    email_info: EmailInfo, // Only to extract which data should be send to the html template
}

#[derive(Deserialize, Serialize)]
struct EmailInfo {
    runner_id: String,
    start_number: String,
    donation: String,
    reason_for_payment: String,
    status_link: String,
}

struct EmailConfiguration {
    sender_email: String,
    smtp_protocol: String,
    credentials: Credentials,
    config_data_provided: bool,
}

pub fn send_registration_email(
    runner_id: String,
    start_number: String,
    receiver_email: String,
    donation: String,
    reason_for_payment: String,
    status_link: String,
) -> bool {
    let email_details = EmailDetails {
        receiver_email,
        template_name: "registration_mail.html",
        subject: "Lauf gegen Rechts - Deine Anmeldung",
        email_info: EmailInfo {
            runner_id,
            start_number,
            donation,
            reason_for_payment,
            status_link,
        },
    };
    send_email_with_subject(email_details)
}

pub fn send_payment_confirmation(
    runner_id: String,
    start_number: String,
    receiver_email: String,
    donation: String,
    status_link: String,
) -> bool {
    let email_details = EmailDetails {
        receiver_email,
        template_name: "payment_confirmation_mail.html",
        subject: "Lauf gegen Rechts - Zahlung bestätigt",
        email_info: EmailInfo {
            runner_id,
            start_number,
            donation,
            reason_for_payment: String::from(""),
            status_link,
        },
    };
    send_email_with_subject(email_details)
}

fn send_email_with_subject(email_details: EmailDetails) -> bool {
    dotenv().ok();
    let email_configuration = get_email_configuration();

    if !email_configuration.config_data_provided {
        return false;
    }

    let mut context = Context::new();

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(_e) => std::process::exit(1),
    };
    let template = web::Data::new(tera);
    context.insert("email_info", &email_details.email_info);
    let rendered = template
        .render(email_details.template_name, &context)
        .unwrap();

    let email_content = Message::builder()
        .from(
            format!("Lauf gegen Rechts<{}>", email_configuration.sender_email)
                .parse()
                .unwrap(),
        )
        .to(email_details.receiver_email.parse().unwrap())
        .subject(email_details.subject)
        .header(header::ContentType::TEXT_HTML)
        .body(rendered)
        .unwrap();

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&email_configuration.smtp_protocol)
        .unwrap()
        .credentials(email_configuration.credentials)
        .build();

    // Send the email
    return match mailer.send(&email_content) {
        Ok(_) => {
            println!("Email sent successfully!");
            true
        }
        Err(_) => false,
    };
}

fn get_email_configuration() -> EmailConfiguration {
    dotenv().ok();
    let sender_email =
        std::env::var("SENDER_EMAIL").unwrap_or_else(|_| "SENDER_EMAIL must be set.".to_string());
    let smtp_password =
        std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "SMTP_PASSWORD must be set.".to_string());
    let smtp_protocol =
        std::env::var("SMTP_PROTOCOL").unwrap_or_else(|_| "SMTP_PROTOCOL must be set.".to_string());

    let config_data_provided = !sender_email.contains("must be set")
        && !smtp_password.contains("must be set")
        && !smtp_protocol.contains("must be set");

    let credentials = Credentials::new(sender_email.to_string(), smtp_password);

    EmailConfiguration {
        sender_email,
        smtp_protocol,
        credentials,
        config_data_provided,
    }
}
