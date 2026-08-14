use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_022143_create_playlist_tracks_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();

        table
            .table("playlist_tracks")
            .if_not_exists()
            .col(
                ColumnDef::new("track_id")
                    .big_unsigned()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new("hashid").string().null())
            .col(ColumnDef::new("playlist_id").big_unsigned().not_null())
            .col(ColumnDef::new("entry_id").big_unsigned().null())
            .col(ColumnDef::new("video_id").big_unsigned().null())
            .col(ColumnDef::new("position").integer().not_null().default(1))
            .col(ColumnDef::new("created_at").timestamp().null())
            .col(ColumnDef::new("updated_at").timestamp().null())
            .foreign_key(
                ForeignKey::create()
                    .name("playlist_tracks_playlist_id_foreign")
                    .from("playlist_tracks", "playlist_id")
                    .to("playlists", "playlist_id")
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("playlist_tracks_entry_id_foreign")
                    .from("playlist_tracks", "entry_id")
                    .to("anime_theme_entries", "entry_id")
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("playlist_tracks_video_id_foreign")
                    .from("playlist_tracks", "video_id")
                    .to("videos", "video_id")
                    .on_delete(ForeignKeyAction::SetNull),
            );

        if manager.get_database_backend() == DbBackend::MySql {
            // Set collation to binary to be case-sensitive
            table.character_set("utf8mb4").collate("utf8mb4_bin");
        }

        manager.create_table(table.to_owned()).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
