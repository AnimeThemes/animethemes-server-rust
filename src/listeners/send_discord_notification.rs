use loco_rs::app::AppContext;
use loco_rs::prelude::BackgroundWorker;
use mod_events::Event;

use crate::AppError;
use crate::enums::features::Feature;
use crate::events::discord::DiscordNotifiableEvent;
use crate::features::functions::FeatureManager;
use crate::workers::discord_notification::{
    DiscordNotificationWorker, DiscordNotificationWorkerArgs,
};

pub async fn handle<E: DiscordNotifiableEvent + Event>(
    ctx: &AppContext,
    event: &E,
) -> Result<(), AppError> {
    let feature_manager = ctx.shared_store.get::<FeatureManager>();

    if let Some(feature_manager) = feature_manager
        && feature_manager.is_enabled(Feature::AllowDiscordNotifications, None, false)
        && event.should_send_discord_message()
    {
        tracing::debug!(
            "Sending Discord notification for event: {}",
            event.event_name()
        );

        DiscordNotificationWorker::perform_later(
            ctx,
            DiscordNotificationWorkerArgs {
                channel_id: event.get_discord_channel(),
                message: serde_json::to_value(event.get_message()?).map_err(AppError::internal)?,
            },
        )
        .await
        .map_err(AppError::internal)?;
    }

    Ok(())
}
