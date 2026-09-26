use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260926_230104_create_theme_staff_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("theme_staff")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("theme_id").big_unsigned().not_null())
                    .col(ColumnDef::new("artist_id").big_unsigned().not_null())
                    .col(ColumnDef::new("role").string().not_null())
                    .col(ColumnDef::new("alias").string().null())
                    .col(ColumnDef::new("relevance").integer().not_null().default(1))
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
                            .name("theme_staff_theme_id_foreign")
                            .from("theme_staff", "theme_id")
                            .to("themes", "theme_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("theme_staff_artist_id_foreign")
                            .from("theme_staff", "artist_id")
                            .to("artists", "artist_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("unique_theme_staff")
                            .col("theme_id")
                            .col("artist_id")
                            .col("role")
                            .col("deleted_at")
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
