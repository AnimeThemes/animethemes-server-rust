use std::env;

use lettre::{Message, message::MultiPart};
use minijinja::{Environment, context};

use crate::mail::{mailer::Mail, messages::Messages};

impl Messages {
    pub fn reset_password_email(name: &str, email: &str, token: &str) -> Message {
        let client_url = env::var("CLIENT_RESET_PASSWORD_URL")
            .expect("CLIENT_RESET_PASSWORD_URL must be set in .env");

        let reset_url = format!("{}{}", client_url, token);

        let mut templates = Environment::new();

        templates
            .add_template(
                "reset-password",
                include_str!("templates/reset-password.html"),
            )
            .expect("failed to load email template");

        let html = templates
            .get_template("reset-password")
            .unwrap()
            .render(context! {
                name => name,
                reset_url => reset_url,
            })
            .expect("failed to render reset password email");

        let text = format!(
            "Hello, {}!\n\n\
            Reset your password:\n\
            {}\n\n\
            Regards,\n\
            AnimeThemes",
            name, reset_url,
        );

        Message::builder()
            .from(Mail::build_from())
            .to(Mail::build_to(name.to_string(), email.to_string()))
            .subject("Reset Password")
            .multipart(MultiPart::alternative_plain_html(text, html))
            .unwrap()
    }
}
