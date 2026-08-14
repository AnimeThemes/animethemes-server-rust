use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260812_231526_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("users")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("name").string().not_null().unique_key())
                    .col(ColumnDef::new("email").string().not_null().unique_key())
                    .col(ColumnDef::new("email_verified_at").timestamp().null())
                    .col(ColumnDef::new("password").string().not_null())
                    .col(ColumnDef::new("two_factor_secret").text().null())
                    .col(ColumnDef::new("two_factor_recovery_codes").text().null())
                    .col(ColumnDef::new("two_factor_confirmed_at").timestamp().null())
                    .col(ColumnDef::new("remember_token").text().null())
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
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
