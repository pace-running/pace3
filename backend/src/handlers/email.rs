use actix_web::web;
use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use dotenvy::dotenv;


#[derive(Deserialize, Serialize)]
struct EmailInfo {
    donation: String,
    reason_for_payment: String,
    status_link: String,
}

pub fn send_registration_email(email: String, donation: String, reason_for_payment: String) -> bool {
    dotenv().ok();
    let sender_email = std::env::var("SENDER_EMAIL").unwrap_or_else(|_| "SENDER_EMAIL must be set.".to_string());
    let smtp_password = std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "SMTP_PASSWORD must be set.".to_string());

    let status_link = "https://pace3.lauf-gegen-rechts.de/".to_string();

    let mut ctx = Context::new();
    let email_info = EmailInfo {
        donation,
        reason_for_payment,
        status_link,
    };
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(_e) => std::process::exit(1),
    };
    let tmpl = web::Data::new(tera);
    ctx.insert("email_info", &email_info);
    let rendered = tmpl.render("registration_mail.html", &ctx).unwrap();
    if sender_email.contains("must be set") || smtp_password.contains("must be set") {
        return false;
    }
    let email = Message::builder()
        .from(
            format!("Lauf gegen Rechts<{}>", sender_email)
                .parse()
                .unwrap(),
        )
        .to(email.parse().unwrap())
        .subject("Lauf gegen Rechts - Deine Anmeldung")
        .header(header::ContentType::TEXT_HTML)
        .body(rendered)
        .unwrap();

    let creds = Credentials::new(sender_email.to_string(), smtp_password.to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email

    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            return true;
        },
        Err(_) => return false
    }
}
