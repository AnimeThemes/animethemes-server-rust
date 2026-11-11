use std::env;

use crate::discord::created_embed;
use crate::entities::list::playlist;
use crate::middlewares::current_user::CurrentUser;

pub struct Playlist {
    playlist: playlist::Model,
    user: Option<CurrentUser>,
}

impl Playlist {
    pub fn new(playlist: playlist::Model, user: Option<CurrentUser>) -> Self {
        Self { playlist, user }
    }

    pub fn created_message(&self) -> serenity::all::CreateMessage {
        let embed = created_embed(self.user.clone()).description(format!(
            "Playlist '**{}**' has been created.",
            self.playlist.name
        ));

        serenity::all::CreateMessage::new().embed(embed)
    }

    pub fn get_discord_channel() -> u64 {
        env::var("DISCORD_ADMIN_CHANNEL_ID")
            .expect("DISCORD_ADMIN_CHANNEL_ID is required in .env")
            .parse()
            .unwrap()
    }
}
