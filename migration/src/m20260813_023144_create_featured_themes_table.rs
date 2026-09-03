use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_023144_create_featured_themes_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("featured_themes")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("featured_theme_id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("start_at").timestamp().not_null())
                    .col(ColumnDef::new("end_at").timestamp().not_null())
                    .col(ColumnDef::new("user_id").big_unsigned().null())
                    .col(ColumnDef::new("entry_id").big_unsigned().null())
                    .col(ColumnDef::new("video_id").big_unsigned().null())
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
                    .foreign_key(
                        ForeignKey::create()
                            .name("featured_themes_user_id_foreign")
                            .from("featured_themes", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("featured_themes_entry_id_foreign")
                            .from("featured_themes", "entry_id")
                            .to("entries", "entry_id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("featured_themes_video_id_foreign")
                            .from("featured_themes", "video_id")
                            .to("videos", "video_id")
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
