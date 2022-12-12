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
    email_info: EmailInfo,
}

#[derive(Deserialize, Serialize)]
struct EmailInfo {
    runner_id: String,
    start_number: String,
    donation: String,
    reason_for_payment: String,
    verification_code: String,
}

struct EmailConfiguration {
    sender_email: String,
    smtp_transport: String,
    credentials: Credentials,
    config_data_provided: bool,
    url_host: String,
}

pub fn send_registration_email(
    runner_id: String,
    start_number: String,
    receiver_email: String,
    donation: String,
    reason_for_payment: String,
    verification_code: String,
    tshirt_cost: String,
) -> bool {
    let email_details = EmailDetails {
        receiver_email,
        template_name: "registration_mail.html",
        subject: "Lauf gegen Rechts - Deine Anmeldung",
        email_info: EmailInfo {
            runner_id,
            start_number,
            donation: (donation.parse::<i32>().unwrap() + tshirt_cost.parse::<i32>().unwrap())
                .to_string(),
            reason_for_payment,
            verification_code,
        },
    };
    send_email_with_subject(email_details)
}

pub fn send_payment_confirmation(
    runner_id: String,
    start_number: String,
    receiver_email: String,
    donation: String,
    verification_code: String,
    tshirt_cost: String,
) -> bool {
    let email_details = EmailDetails {
        receiver_email,
        template_name: "payment_confirmation_mail.html",
        subject: "Lauf gegen Rechts - Zahlung bestätigt",
        email_info: EmailInfo {
            runner_id,
            start_number,
            donation: (donation.parse::<i32>().unwrap() + tshirt_cost.parse::<i32>().unwrap())
                .to_string(),
            reason_for_payment: String::from(""),
            verification_code,
        },
    };
    send_email_with_subject(email_details)
}

fn send_email_with_subject(email_details: EmailDetails) -> bool {
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
    context.insert("url_host", &email_configuration.url_host);
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
    let mailer = SmtpTransport::relay(&email_configuration.smtp_transport)
        .unwrap()
        .credentials(email_configuration.credentials)
        .build();

    // Send the email
    match mailer.send(&email_content) {
        Ok(_) => {
            println!("Email sent successfully!");
            true
        }
        Err(err) => {
            println!("Failed to send mail: {}", err);
            false
        }
    }
}

fn get_email_configuration() -> EmailConfiguration {
    dotenv().ok();
    let sender_email =
        std::env::var("SENDER_EMAIL").unwrap_or_else(|_| "SENDER_EMAIL must be set.".to_string());
    let smtp_user =
        std::env::var("SMTP_USER").unwrap_or_else(|_| "SMTP_USER must be set.".to_string());
    let smtp_password =
        std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "SMTP_PASSWORD must be set.".to_string());
    let smtp_transport = std::env::var("SMTP_TRANSPORT")
        .unwrap_or_else(|_| "SMTP_TRANSPORT must be set.".to_string());
    let url_host =
        std::env::var("URL_HOST").unwrap_or_else(|_| "URL_HOST must be set.".to_string());

    let config_data_provided = !sender_email.contains("must be set")
        && !smtp_password.contains("must be set")
        && !smtp_user.contains("must be set")
        && !smtp_transport.contains("must be set")
        && !url_host.contains("must be set");

    let credentials = Credentials::new(smtp_user, smtp_password);

    EmailConfiguration {
        sender_email,
        smtp_transport,
        credentials,
        config_data_provided,
        url_host,
    }
}
