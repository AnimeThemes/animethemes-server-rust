use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261122_144654_modify_resources_schema"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager.has_column("resources", "resourceable_id").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table("resources")
                        .add_column(ColumnDef::new("resourceable_type").string().not_null())
                        .add_column(ColumnDef::new("resourceable_id").big_unsigned().not_null())
                        .add_column(ColumnDef::new("as").string().null())
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .name("resources_resourceable_index")
                        .table("resources")
                        .col("resourceable_type")
                        .col("resourceable_id")
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
