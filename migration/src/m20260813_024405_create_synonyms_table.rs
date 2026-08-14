use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_024405_create_synonyms_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("synonyms")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("synonym_id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("synonymable_type").string().not_null())
                    .col(ColumnDef::new("synonymable_id").big_unsigned().not_null())
                    .col(ColumnDef::new("language").string().null())
                    .col(ColumnDef::new("text").string().not_null())
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
                    .col(ColumnDef::new("deleted_at").timestamp().null())
                    .index(
                        Index::create()
                            .name("synonyms_synonymable_type_synonymable_id_index")
                            .col("synonymable_type")
                            .col("synonymable_id"),
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
