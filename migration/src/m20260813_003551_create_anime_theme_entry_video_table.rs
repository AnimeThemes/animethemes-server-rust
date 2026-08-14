use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_003551_create_anime_theme_entry_video_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("anime_theme_entry_video")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("entry_id").big_unsigned().not_null())
                    .col(ColumnDef::new("video_id").big_unsigned().not_null())
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("anime_theme_entry_video_entry_id_foreign")
                            .from("anime_theme_entry_video", "entry_id")
                            .to("anime_theme_entries", "entry_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("anime_theme_entry_video_video_id_foreign")
                            .from("anime_theme_entry_video", "video_id")
                            .to("videos", "video_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("entry_video_unique_index")
                            .col("entry_id")
                            .col("video_id")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
