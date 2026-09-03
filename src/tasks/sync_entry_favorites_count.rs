use loco_rs::prelude::*;
use sea_orm::{DatabaseBackend, Statement};

pub struct SyncEntryFavoritesCount;

#[async_trait]
impl Task for SyncEntryFavoritesCount {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "entry:sync-favorites-count".to_string(),
            detail: "Synchronizes likes for the entry entity".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        app_context
            .db
            .execute_raw(Statement::from_sql_and_values(
                DatabaseBackend::MySql,
                r#"
                    UPDATE anime_theme_entries AS entries
                    LEFT JOIN (
                        SELECT
                            favoriteable_id,
                            COUNT(*) AS total
                        FROM favorites
                        WHERE favoriteable_type = 'entry'
                        GROUP BY favoriteable_id
                    ) AS favorite_counts
                        ON favorite_counts.favoriteable_id = entries.entry_id
                    SET entries.favorites_count = COALESCE(favorite_counts.total, 0)
                "#,
                [],
            ))
            .await?;

        Ok(())
    }
}
