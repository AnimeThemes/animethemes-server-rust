use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_002131_create_anime_themes_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("anime_themes")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("theme_id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("anime_id").big_unsigned().not_null())
                    .col(ColumnDef::new("type").integer().not_null())
                    .col(ColumnDef::new("sequence").integer().null())
                    .col(ColumnDef::new("slug").string().not_null())
                    .col(ColumnDef::new("song_id").big_unsigned().null())
                    .col(ColumnDef::new("group_id").big_unsigned().null())
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
                    .col(ColumnDef::new("deleted_at").timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("anime_themes_anime_id_foreign")
                            .from("anime_themes", "anime_id")
                            .to("anime", "anime_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("anime_themes_song_id_foreign")
                            .from("anime_themes", "song_id")
                            .to("songs", "song_id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("anime_themes_group_id_foreign")
                            .from("anime_themes", "group_id")
                            .to("groups", "group_id")
                            .on_delete(ForeignKeyAction::SetNull),
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
