use std::env;

use loco_rs::Result;

use crate::discord::{created_embed, updated_embed};
use crate::entities::auth::user::{self};
use crate::middlewares::current_user::CurrentUser;

pub struct User {
    user: user::Model,
    auth_user: Option<CurrentUser>,
}

impl User {
    pub fn new(user: user::Model, auth_user: Option<CurrentUser>) -> Self {
        Self { user, auth_user }
    }

    pub fn created_message(&self) -> serenity::all::CreateMessage {
        let embed = created_embed(self.auth_user.clone())
            .description(format!("User '**{}**' has been created.", self.user.name));

        serenity::all::CreateMessage::new().embed(embed)
    }

    pub fn updated_message(&self, new_user: user::Model) -> Result<serenity::all::CreateMessage> {
        let embed = updated_embed::<user::Entity>(self.auth_user.clone(), &self.user, &new_user)
            .unwrap()
            .description(format!("User '**{}**' has been updated.", self.user.name));

        Ok(serenity::all::CreateMessage::new().embed(embed))
    }

    pub fn get_discord_channel() -> u64 {
        env::var("DISCORD_ADMIN_CHANNEL_ID")
            .expect("DISCORD_ADMIN_CHANNEL_ID is required in .env")
            .parse()
            .unwrap()
    }
}
