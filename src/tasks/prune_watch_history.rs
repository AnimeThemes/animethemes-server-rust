use chrono::Duration;
use chrono::Utc;
use loco_rs::Error as LocoError;
use loco_rs::prelude::*;

use crate::entities::user::watchhistory;

pub struct PruneWatchHistory;

#[async_trait]
impl Task for PruneWatchHistory {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "prune:watch-history".to_string(),
            detail: "Prune watch history for all users".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        watchhistory::Entity::delete_many()
            .filter(watchhistory::Column::CreatedAt.lt(Utc::now() - Duration::weeks(1)))
            .exec(&app_context.db)
            .await
            .map_err(LocoError::DB)?;

        tracing::info!("Task PruneWatchHistory executed");

        Ok(())
    }
}
