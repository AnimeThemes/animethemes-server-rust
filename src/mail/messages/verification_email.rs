use lettre::{Message, message::MultiPart};
use minijinja::{Environment, context};

use crate::mail::{mailer::Mail, messages::Messages};

impl Messages {
    pub fn verification_email(name: &str, email: &str, verification_url: &str) -> Message {
        let mut templates = Environment::new();

        templates
            .add_template(
                "verification",
                include_str!("templates/verification.html"),
            )
            .expect("failed to load verification email template");

        let html = templates
            .get_template("verification")
            .unwrap()
            .render(context! {
                name => name,
                verification_url => verification_url,
            })
            .expect("failed to render verification email");

        let text = format!(
            "Hello, {}!\n\n\
            Please verify your email address:\n\
            {}\n\n\
            If you did not create an account, no further action is required.\n\n\
            Regards,\n\
            AnimeThemes",
            name, verification_url,
        );

        Message::builder()
            .from(Mail::build_from())
            .to(Mail::build_to(name.to_string(), email.to_string()))
            .subject("Verify Email Address")
            .multipart(MultiPart::alternative_plain_html(text, html))
            .unwrap()
    }
}
