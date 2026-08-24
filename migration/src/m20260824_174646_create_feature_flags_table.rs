use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260824_174646_create_feature_flags_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("feature_flags")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("name").string().not_null())
                    .col(ColumnDef::new("scope_type").string().not_null())
                    .col(ColumnDef::new("scope_id").big_unsigned().null())
                    .col(ColumnDef::new("value").text().not_null())
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
                    .index(
                        Index::create()
                            .name("scope_index")
                            .col("scope_type")
                            .col("scope_id"),
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
