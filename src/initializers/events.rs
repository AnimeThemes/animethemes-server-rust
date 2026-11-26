use std::sync::Arc;

use mod_events::EventDispatcher;

use crate::events::auth::user_registered::UserRegistered;
use crate::events::auth::user_updated::UserUpdated;
use crate::events::list::playlist_created::PlaylistCreated;
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

    listen!(dispatcher, PlaylistCreated => [
        listeners::send_discord_notification::handle,
    ]);

    dispatcher
}
