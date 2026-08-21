use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_024141_create_imageables_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("imageables")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("image_id").big_unsigned().not_null())
                    .col(ColumnDef::new("imageable_type").string().not_null())
                    .col(ColumnDef::new("imageable_id").big_unsigned().not_null())
                    .col(ColumnDef::new("depth").integer().not_null().default(1))
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
                            .name("resourceables_resource_id_foreign")
                            .from("imageables", "image_id")
                            .to("images", "image_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("imageables_imageable_type_imageable_id_index")
                            .col("imageable_type")
                            .col("imageable_id"),
                    )
                    .index(
                        Index::create()
                            .name("imageables_image_id_foreign")
                            .col("image_id")
                            .col("imageable_type")
                            .col("imageable_id")
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
