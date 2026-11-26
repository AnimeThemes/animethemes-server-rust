use mod_events::Event;

use crate::AppError;
use crate::entities::list::playlist;
use crate::events::discord::{DiscordNotifiable, created_embed};
use crate::middlewares::current_user::CurrentUser;

#[derive(Debug, Clone)]
pub struct PlaylistCreated {
    pub playlist: playlist::Model,
    pub user: Option<CurrentUser>,
}

impl Event for PlaylistCreated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl DiscordNotifiable for PlaylistCreated {
    fn get_discord_channel(&self) -> u64 {
        std::env::var("DISCORD_ADMIN_CHANNEL_ID")
            .expect("DISCORD_ADMIN_CHANNEL_ID is required in .env")
            .parse()
            .unwrap()
    }

    fn get_message(&self) -> Result<serenity::all::CreateMessage, AppError> {
        let embed = created_embed(self.user.clone()).description(format!(
            "Playlist '**{}**' has been created.",
            self.playlist.name
        ));

        Ok(serenity::all::CreateMessage::new().embed(embed))
    }
}
