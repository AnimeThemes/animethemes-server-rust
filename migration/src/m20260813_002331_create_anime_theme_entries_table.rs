use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_002331_create_anime_theme_entries_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("anime_theme_entries")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("entry_id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("theme_id").big_unsigned().not_null())
                    .col(ColumnDef::new("version").integer().not_null())
                    .col(ColumnDef::new("episodes").string().null())
                    .col(ColumnDef::new("nsfw").boolean().default(false))
                    .col(ColumnDef::new("spoiler").boolean().default(false))
                    .col(ColumnDef::new("notes").text().null())
                    .col(
                        ColumnDef::new("likes_count")
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new("tracks_count")
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new("created_at")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new("updated_at")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new("deleted_at").timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("anime_theme_entries_theme_id_foreign")
                            .from("anime_theme_entries", "theme_id")
                            .to("anime_themes", "theme_id")
                            .on_delete(ForeignKeyAction::Cascade),
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
