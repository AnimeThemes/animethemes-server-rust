use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261122_180303_modify_images_schema"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager.has_column("images", "imageable_id").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table("images")
                        .add_column(ColumnDef::new("imageable_type").string().null())
                        .add_column(ColumnDef::new("imageable_id").big_unsigned().null())
                        .add_column(ColumnDef::new("depth").integer().not_null().default(1))
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .name("images_imageable_index")
                        .table("images")
                        .col("imageable_type")
                        .col("imageable_id")
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
