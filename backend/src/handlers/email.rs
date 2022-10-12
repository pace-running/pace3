use actix_web::web;
use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Deserialize, Serialize)]
struct EmailInfo {
    donation: String,
    reason_for_payment: String,
}

pub fn send_registration_email(email: String, donation: String, reason_for_payment: String) {
    // enter smtp credentials here
    let sender_email = "";
    let smtp_password = "";

    let mut ctx = Context::new();
    let email_info = EmailInfo {
        donation,
        reason_for_payment,
    };
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(_e) => std::process::exit(1),
    };
    let tmpl = web::Data::new(tera);
    ctx.insert("info", &email_info);
    let rendered = tmpl.render("registration_mail.html", &ctx).unwrap();

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
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
