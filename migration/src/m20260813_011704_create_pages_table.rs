use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_011704_create_pages_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("pages")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("page_id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("name").string().not_null())
                    .col(ColumnDef::new("slug").string().not_null())
                    .col(ColumnDef::new("body").custom("MEDIUMTEXT").not_null())
                    .col(ColumnDef::new("previous_id").big_unsigned().null())
                    .col(ColumnDef::new("next_id").big_unsigned().null())
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
                    .col(ColumnDef::new("deleted_at").timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pages_previous_id_foreign")
                            .from("pages", "previous_id")
                            .to("pages", "page_id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("pages_next_id_foreign")
                            .from("pages", "next_id")
                            .to("pages", "page_id")
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
