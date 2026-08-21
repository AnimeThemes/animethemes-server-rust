use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_000608_create_anime_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("anime")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("anime_id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("title").string().not_null())
                    .col(ColumnDef::new("title_english").string().null())
                    .col(ColumnDef::new("title_native").string().null())
                    .col(ColumnDef::new("slug").string().not_null())
                    .col(ColumnDef::new("year").integer().null())
                    .col(ColumnDef::new("start_date").char_len(8).null())
                    .col(ColumnDef::new("end_date").char_len(8).null())
                    .col(ColumnDef::new("season").integer().null())
                    .col(ColumnDef::new("format").integer().null())
                    .col(ColumnDef::new("synopsis").text().null())
                    .col(ColumnDef::new("mod_notes").text().null())
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
