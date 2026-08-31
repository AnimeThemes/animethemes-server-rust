use std::env;

use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::Mailbox};

use reqwest::multipart::{Form, Part};

use crate::AppError;

enum MailBackend {
    Mailpit,
    Mailgun,
}

impl MailBackend {
    pub async fn send(&self, message: Message) -> Result<(), AppError> {
        match self {
            MailBackend::Mailpit => {
                let host = env::var("MAIL_HOST").expect("MAIL_HOST must be set");

                let port = env::var("MAIL_PORT")
                    .unwrap_or_else(|_| "1025".to_string())
                    .parse::<u16>()
                    .unwrap_or(1025);

                let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host)
                    .port(port)
                    .build();

                mailer.send(message).await.map_err(AppError::internal)?;

                Ok(())
            }
            MailBackend::Mailgun => {
                let domain =
                    env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN must be set in .env");
                let secret =
                    env::var("MAILGUN_SECRET").expect("MAILGUN_SECRET must be set in .env");
                let endpoint =
                    env::var("MAILGUN_ENDPOINT").unwrap_or_else(|_| "api.mailgun.net".to_string());

                let url = format!("https://{}/v3/{}/messages.mime", endpoint, domain);

                let recipients = message
                    .envelope()
                    .to()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();

                let mime = message.formatted();

                let part = Part::bytes(mime)
                    .file_name("message.eml")
                    .mime_str("message/rfc822")
                    .map_err(AppError::internal)?;

                let mut form = Form::new().part("message", part);

                for recipient in recipients {
                    form = form.text("to", recipient);
                }

                let response = reqwest::Client::builder()
                    .build()
                    .map_err(AppError::internal)?
                    .post(url)
                    .basic_auth("api", Some(secret))
                    .multipart(form)
                    .send()
                    .await
                    .map_err(AppError::internal)?;

                response.error_for_status().map_err(AppError::internal)?;

                Ok(())
            }
        }
    }
}

pub struct Mail {}

impl Mail {
    fn get_backend() -> MailBackend {
        match env::var("MAIL_MAILER")
            .expect("MAIL_HOST must be set in .env")
            .as_str()
        {
            "smtp" => MailBackend::Mailpit,
            "mailgun" => MailBackend::Mailgun,
            _ => MailBackend::Mailpit,
        }
    }

    pub fn build_from() -> Mailbox {
        let name = env::var("MAIL_FROM_NAME").expect("MAIL_FROM_NAME must be set");
        let email = env::var("MAIL_FROM_ADDRESS").expect("MAIL_FROM_ADDRESS must be set");

        Mailbox::new(Some(name), email.parse().unwrap())
    }

    pub fn build_to(name: String, email: String) -> Mailbox {
        Mailbox::new(Some(name), email.parse().unwrap())
    }

    pub async fn send(message: Message) -> Result<(), AppError> {
        Self::get_backend().send(message).await?;

        Ok(())
    }
}
