use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_023839_create_resourceables_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("resourceables")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("resource_id").big_unsigned().not_null())
                    .col(ColumnDef::new("resourceable_type").string().not_null())
                    .col(ColumnDef::new("resourceable_id").big_unsigned().not_null())
                    .col(ColumnDef::new("as").string().null())
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("resourceables_resource_id_foreign")
                            .from("resourceables", "resource_id")
                            .to("resources", "resource_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("resourceables_resourceable_type_resourceable_id_index")
                            .col("resourceable_type")
                            .col("resourceable_id"),
                    )
                    .index(
                        Index::create()
                            .name("resourceables_unique_index")
                            .col("resource_id")
                            .col("resourceable_type")
                            .col("resourceable_id")
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
