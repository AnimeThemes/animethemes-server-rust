use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260828_130923_create_sanctions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("sanctions")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("name").string().not_null())
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
                            .name("roles_name_unique")
                            .col("name")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("user_sanctions")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("user_id").big_unsigned().null())
                    .col(ColumnDef::new("sanction_id").big_unsigned().null())
                    .col(ColumnDef::new("moderator_id").big_unsigned().null())
                    .col(ColumnDef::new("expires_at").timestamp().null())
                    .col(ColumnDef::new("reason").text().null())
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
                            .name("user_sanctions_user_id_foreign")
                            .from("user_sanctions", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_sanctions_sanction_id_foreign")
                            .from("user_sanctions", "sanction_id")
                            .to("sanctions", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_sanctions_moderator_id_foreign")
                            .from("user_sanctions", "moderator_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .index(
                        Index::create()
                            .name("user_sanctions_user_id_sanction_id_index")
                            .col("user_id")
                            .col("sanction_id")
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("user_sanctions_moderator_id_index")
                            .col("moderator_id"),
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
