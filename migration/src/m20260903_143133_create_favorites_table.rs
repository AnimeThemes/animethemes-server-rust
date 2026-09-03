use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260903_143133_create_favorites_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("favorites")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("user_id").big_unsigned().not_null())
                    .col(ColumnDef::new("favoriteable_type").string().not_null())
                    .col(ColumnDef::new("favoriteable_id").big_unsigned().not_null())
                    .col(
                        ColumnDef::new("created_at")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .index(
                        Index::create()
                            .name("favorites_favoriteable_type_favoriteable_id_index")
                            .col("favoriteable_type")
                            .col("favoriteable_id"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("favorites_user_id_foreign")
                            .from("favorites", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("favorites_index")
                            .col("favoriteable_type")
                            .col("favoriteable_id")
                            .col("user_id"),
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
