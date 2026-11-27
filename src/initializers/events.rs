use std::sync::Arc;

use mod_events::EventDispatcher;

use crate::events::auth::user_registered::UserRegistered;
use crate::events::auth::user_updated::UserUpdated;
use crate::events::auth::user_verified::UserVerified;
use crate::events::content::anime_created::AnimeCreated;
use crate::events::content::anime_deleting::AnimeDeleting;
use crate::events::content::anime_updated::AnimeUpdated;
use crate::events::content::artist_created::ArtistCreated;
use crate::events::content::artist_deleting::ArtistDeleting;
use crate::events::content::artist_updated::ArtistUpdated;
use crate::events::content::series_created::SeriesCreated;
use crate::events::content::series_deleting::SeriesDeleting;
use crate::events::content::series_updated::SeriesUpdated;
use crate::events::content::song_created::SongCreated;
use crate::events::content::song_deleting::SongDeleting;
use crate::events::content::song_updated::SongUpdated;
use crate::events::content::studio_created::StudioCreated;
use crate::events::content::studio_deleting::StudioDeleting;
use crate::events::content::studio_updated::StudioUpdated;
use crate::events::list::playlist_created::PlaylistCreated;
use crate::events::list::playlist_deleted::PlaylistDeleted;
use crate::events::list::playlist_updated::PlaylistUpdated;
use crate::listeners;

macro_rules! listen {
    ($dispatcher:expr, $event:ty => [$($handler:path),* $(,)?]) => {
        $(
            $dispatcher.subscribe_async(
                |envelope: &crate::events::EventContext<$event>| {
                    let ctx = envelope.ctx.clone();
                    let event = envelope.event.clone();

                    async move {
                        $handler(&ctx, &event)
                            .await
                            .map_err(mod_events::ListenerError::new)
                    }
                }
            );
        )*
    };
}

pub fn register() -> Arc<EventDispatcher> {
    let dispatcher = Arc::new(EventDispatcher::new());

    listen!(dispatcher, UserRegistered => [
        listeners::send_verification_email::handle,
        listeners::send_discord_notification::handle,
    ]);

    listen!(dispatcher, UserUpdated => [
        listeners::send_discord_notification::handle,
    ]);

    listen!(dispatcher, UserVerified => [
        listeners::verified::assign_verified_role,
    ]);

    listen!(dispatcher, PlaylistCreated => [
        listeners::update_typesense_documents::handle,
        listeners::send_discord_notification::handle,
    ]);

    listen!(dispatcher, PlaylistDeleted => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, PlaylistUpdated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, AnimeCreated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, AnimeDeleting => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, AnimeUpdated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, ArtistCreated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, ArtistDeleting => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, ArtistUpdated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, SeriesCreated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, SeriesDeleting => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, SeriesUpdated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, SongCreated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, SongDeleting => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, SongUpdated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, StudioCreated => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, StudioDeleting => [
        listeners::update_typesense_documents::handle,
    ]);

    listen!(dispatcher, StudioUpdated => [
        listeners::update_typesense_documents::handle,
    ]);

    dispatcher
}
