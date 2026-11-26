use mod_events::Event;

use crate::AppError;
use crate::entities::auth::user;
use crate::events::discord::{DiscordNotifiable, updated_embed};
use crate::middlewares::current_user::CurrentUser;

#[derive(Debug, Clone)]
pub struct UserUpdated {
    pub old_user: user::Model,
    pub new_user: user::Model,
    pub auth_user: Option<CurrentUser>,
}

impl Event for UserUpdated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl DiscordNotifiable for UserUpdated {
    fn get_discord_channel(&self) -> u64 {
        std::env::var("DISCORD_ADMIN_CHANNEL_ID")
            .expect("DISCORD_ADMIN_CHANNEL_ID is required in .env")
            .parse()
            .unwrap()
    }

    fn get_message(&self) -> Result<serenity::all::CreateMessage, AppError> {
        let embed =
            updated_embed::<user::Entity>(self.auth_user.clone(), &self.old_user, &self.new_user)
                .unwrap()
                .description(format!(
                    "User '**{}**' has been updated.",
                    self.new_user.name
                ));

        Ok(serenity::all::CreateMessage::new().embed(embed))
    }
}
