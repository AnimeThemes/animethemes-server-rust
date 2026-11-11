use loco_rs::app::AppContext;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, Http};

pub struct DiscordNotificationWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DiscordNotificationWorkerArgs {
    pub channel_id: u64,
    pub message: serde_json::Value,
}

#[async_trait]
impl BackgroundWorker<DiscordNotificationWorkerArgs> for DiscordNotificationWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: DiscordNotificationWorkerArgs) -> Result<()> {
        let token = std::env::var("DISCORD_BOT_TOKEN").unwrap();

        let http = Http::new(&token);

        http.send_message(ChannelId::new(args.channel_id), Vec::new(), &args.message)
            .await
            .map_err(Error::wrap)?;

        Ok(())
    }
}
