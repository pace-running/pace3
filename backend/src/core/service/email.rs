use crate::models::runner::Runner;
use lettre::transport::smtp::client::{Certificate, Tls, TlsParametersBuilder};
use log::warn;
#[cfg(test)]
use mockall::automock;
use std::env::VarError;

#[cfg_attr(test, automock)]
pub trait EmailService {
    fn send_registration_confirmation(&self, runner: Runner) -> anyhow::Result<()>;
    fn send_payment_confirmation(&self, runner: Runner) -> anyhow::Result<()>;
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct EmailConfiguration {
    sender_email_address: String,
    sender_display_name: Option<String>,
    smtp_relay_server_domain: String,
    smtp_relay_server_port: Option<u16>,
    smtp_username: String,
    smtp_password: String,
    root_cert_path: Option<String>,
}

impl EmailConfiguration {
    pub fn new(
        sender_email_address: String,
        sender_display_name: Option<String>,
        smtp_relay_server_domain: String,
        smtp_relay_server_port: Option<u16>,
        smtp_username: String,
        smtp_password: String,
        root_cert_path: Option<String>,
    ) -> Self {
        EmailConfiguration {
            sender_email_address,
            sender_display_name,
            smtp_relay_server_domain,
            smtp_relay_server_port,
            smtp_username,
            smtp_password,
            root_cert_path,
        }
    }

    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let sender_email_address = std::env::var("SENDER_EMAIL")?;
        let sender_display_name = std::env::var("SENDER_DISPLAY_NAME")
            .map(Some)
            .or_else(|e| match e {
                VarError::NotPresent => Ok(Some("Lauf gegen Rechts".to_string())),
                VarError::NotUnicode(_) => Err(e),
            })?;
        let smtp_username = std::env::var("SMTP_USER")?;
        let smtp_password = std::env::var("SMTP_PASSWORD")?; // talisman-ignore-line
        let smtp_relay_server_domain = std::env::var("SMTP_TRANSPORT")?;
        let smtp_relay_server_port = std::env::var("SMTP_PORT").map_or_else(
            |e| match e {
                VarError::NotPresent => Ok(None),
                VarError::NotUnicode(_) => Err(anyhow::Error::from(e)),
            },
            |v| v.parse().map_err(anyhow::Error::from).map(Some),
        )?;

        let root_cert_path =
            std::env::var("SMTP_TLS_ROOT_CERT_PATH")
                .map(Some)
                .or_else(|e| match e {
                    VarError::NotPresent => Ok(None),
                    VarError::NotUnicode(_) => Err(e),
                })?;

        Ok(Self {
            sender_email_address,
            sender_display_name,
            smtp_relay_server_domain,
            smtp_relay_server_port,
            smtp_username,
            smtp_password,
            root_cert_path,
        })
    }
}

#[derive(Clone)]
pub struct LettreConfiguration {
    sender_mailbox: lettre::message::Mailbox,
    smtp_transport: lettre::transport::smtp::SmtpTransport,
}

impl TryFrom<EmailConfiguration> for LettreConfiguration {
    type Error = anyhow::Error;

    fn try_from(email_configuration: EmailConfiguration) -> Result<Self, Self::Error> {
        let sender_mailbox = lettre::message::Mailbox::new(
            email_configuration.sender_display_name,
            email_configuration.sender_email_address.parse()?,
        );
        let credentials = lettre::transport::smtp::authentication::Credentials::new(
            email_configuration.smtp_username,
            email_configuration.smtp_password,
        );

        let mut smtp_transport_builder =
            lettre::SmtpTransport::relay(&email_configuration.smtp_relay_server_domain)?
                .credentials(credentials);

        if let Some(root_cert_path) = email_configuration.root_cert_path {
            let pem_cert = std::fs::read(root_cert_path).unwrap();
            let certificate = Certificate::from_pem(&pem_cert).unwrap();
            let tls_parameters =
                TlsParametersBuilder::new(email_configuration.smtp_relay_server_domain.clone())
                    .add_root_certificate(certificate)
                    .build()
                    .unwrap();

            smtp_transport_builder = smtp_transport_builder.tls(Tls::Opportunistic(tls_parameters));
        }

        let smtp_transport = if let Some(port) = email_configuration.smtp_relay_server_port {
            smtp_transport_builder.port(port).build()
        } else {
            smtp_transport_builder.build()
        };

        Ok(LettreConfiguration {
            sender_mailbox,
            smtp_transport,
        })
    }
}

pub struct LettreTeraEmailService {
    lettre_configuration: LettreConfiguration,
    templates: &'static tera::Tera,
    application_domain: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct TeraEmailInfo {
    runner_id: String,
    start_number: String,
    donation: String,
    reason_for_payment: String,
    verification_code: String,
}

impl From<Runner> for TeraEmailInfo {
    fn from(runner: Runner) -> Self {
        TeraEmailInfo {
            runner_id: runner.id.to_string(),
            start_number: runner.start_number.to_string(),
            donation: (runner.donation.parse::<i32>().unwrap()
                + runner.tshirt_cost.parse::<i32>().unwrap())
            .to_string(),
            reason_for_payment: runner.reason_for_payment,
            verification_code: runner.verification_code,
        }
    }
}

impl From<&Runner> for TeraEmailInfo {
    fn from(runner: &Runner) -> Self {
        TeraEmailInfo {
            runner_id: runner.id.to_string(),
            start_number: runner.start_number.to_string(),
            donation: (runner.donation.parse::<i32>().unwrap()
                + runner.tshirt_cost.parse::<i32>().unwrap())
            .to_string(),
            reason_for_payment: runner.reason_for_payment.to_string(),
            verification_code: runner.verification_code.to_string(),
        }
    }
}

impl LettreTeraEmailService {
    pub fn new(
        email_configuration: EmailConfiguration,
        templates: &'static tera::Tera,
        application_domain: Option<String>,
    ) -> anyhow::Result<Self> {
        let application_domain = application_domain
            .ok_or_else(|| anyhow::Error::msg("Value for `application_domain` neither provided as parameter nor as env variable `URL_HOST`"))
            .or_else(|e| std::env::var("URL_HOST").map_err(|var_error| match var_error {
                VarError::NotPresent => e,
                VarError::NotUnicode(_) => anyhow::Error::from(var_error),
            }))?;
        let lettre_configuration = email_configuration.try_into()?;
        Ok(LettreTeraEmailService {
            lettre_configuration,
            templates,
            application_domain,
        })
    }

    fn send_email(
        &self,
        receiver_email: &str,
        subject: &str,
        template_name: &str,
        context: &mut tera::Context,
    ) -> anyhow::Result<()> {
        use lettre::Transport;

        let body = self.templates.render(template_name, context)?;

        let email_content = lettre::message::Message::builder()
            .from(self.lettre_configuration.sender_mailbox.clone())
            .to(receiver_email.parse()?)
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(body)?;

        let mailer = self.lettre_configuration.smtp_transport.clone();

        let response = mailer.send(&email_content)?;

        return if response.is_positive() {
            Ok(())
        } else {
            let response_code = response.code();
            let error_message = if let Some(first_line) = response.first_line() {
                format!(
                    "Failed to send message. Response was {}: {}",
                    response_code, first_line,
                )
            } else {
                format!(
                    "Failed to send message. Response was {}: [no response message provided]",
                    response_code,
                )
            };

            Err(anyhow::Error::msg(error_message))
        };
    }
}

impl EmailService for LettreTeraEmailService {
    fn send_registration_confirmation(&self, runner: Runner) -> anyhow::Result<()> {
        let mut context = tera::Context::new();
        context.insert("url_host", &self.application_domain);
        let email_info: TeraEmailInfo = (&runner).into();
        context.insert("email_info", &email_info);

        self.send_email(
            &runner.email.ok_or_else(|| {
                anyhow::Error::msg(format!(
                    "Runner {} does not have an email address",
                    runner.id
                ))
            })?,
            "Lauf gegen Rechts - Deine Anmeldung",
            "registration_mail.html",
            &mut context,
        )
    }

    fn send_payment_confirmation(&self, runner: Runner) -> anyhow::Result<()> {
        let mut context = tera::Context::new();
        context.insert("url_host", &self.application_domain);
        let email_info: TeraEmailInfo = (&runner).into();
        context.insert("email_info", &email_info);

        self.send_email(
            &runner.email.ok_or_else(|| {
                anyhow::Error::msg(format!(
                    "Runner {} does not have an email address",
                    runner.id
                ))
            })?,
            "Lauf gegen Rechts - Zahlung bestätigt",
            "payment_confirmation_mail.html",
            &mut context,
        )
    }
}

pub struct NonfunctionalEmailService {}

impl EmailService for NonfunctionalEmailService {
    fn send_registration_confirmation(&self, runner: Runner) -> anyhow::Result<()> {
        warn!("Did not send registration confirmation email to runner {}, as nonfunctional implementation is used", runner.id);

        Ok(())
    }

    fn send_payment_confirmation(&self, runner: Runner) -> anyhow::Result<()> {
        warn!("Did not send payment confirmation email to runner {}, as nonfunctional implementation is used", runner.id);

        Ok(())
    }
}
