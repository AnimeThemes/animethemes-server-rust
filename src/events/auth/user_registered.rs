use mod_events::Event;

use crate::AppError;
use crate::entities::auth::user;
use crate::events::discord::{DiscordNotifiableEvent, created_embed};
use crate::middlewares::current_user::CurrentUser;

#[derive(Debug, Clone)]
pub struct UserRegistered {
    pub user: user::Model,
    pub auth_user: Option<CurrentUser>,
}

impl Event for UserRegistered {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl DiscordNotifiableEvent for UserRegistered {
    fn get_discord_channel(&self) -> u64 {
        std::env::var("DISCORD_ADMIN_CHANNEL_ID")
            .expect("DISCORD_ADMIN_CHANNEL_ID is required in .env")
            .parse()
            .unwrap()
    }

    fn get_message(&self) -> Result<serenity::all::CreateMessage, AppError> {
        let embed = created_embed(self.auth_user.clone())
            .description(format!("User '**{}**' has been created.", self.user.name));

        Ok(serenity::all::CreateMessage::new().embed(embed))
    }
}
