use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::list::playlist;
use crate::events::discord::{DiscordNotifiableEvent, created_embed};
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

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

impl UpdateTypesenseDocuments for PlaylistCreated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_playlist(&ctx.db, vec![self.playlist.id]).await?;

        Ok(())
    }
}

impl DiscordNotifiableEvent for PlaylistCreated {
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
