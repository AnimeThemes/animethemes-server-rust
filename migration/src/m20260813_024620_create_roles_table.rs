use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_024620_create_roles_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("roles")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("name").string().not_null())
                    .col(ColumnDef::new("guard_name").string().not_null())
                    .col(
                        ColumnDef::new("default")
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new("color").string().null())
                    .col(ColumnDef::new("priority").integer().not_null().default(0))
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
                            .name("roles_name_guard_name_unique")
                            .col("name")
                            .col("guard_name")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("model_has_roles")
                    .if_not_exists()
                    .col(ColumnDef::new("role_id").big_unsigned().not_null())
                    .col(ColumnDef::new("model_type").string().not_null())
                    .col(ColumnDef::new("model_id").big_unsigned().not_null())
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
                    .primary_key(
                        Index::create()
                            .col("role_id")
                            .col("model_type")
                            .col("model_id"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("model_has_roles_role_id_foreign")
                            .from("model_has_roles", "role_id")
                            .to("roles", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("model_has_roles_model_id_model_type_index")
                            .col("model_type")
                            .col("model_id")
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
