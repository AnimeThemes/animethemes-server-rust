use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_021040_create_playlists_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();

        table
            .table("playlists")
            .if_not_exists()
            .col(
                ColumnDef::new("playlist_id")
                    .big_unsigned()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new("hashid").string().null())
            .col(ColumnDef::new("name").string().not_null())
            .col(ColumnDef::new("visibility").integer().not_null())
            .col(ColumnDef::new("description").text().null())
            .col(ColumnDef::new("user_id").big_unsigned().not_null())
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
                    .name("playlists_user_id_foreign")
                    .from("playlists", "user_id")
                    .to("users", "id")
                    .on_delete(ForeignKeyAction::Cascade),
            );

        if manager.get_database_backend() == DbBackend::MySql {
            // Set collation to binary to be case-sensitive
            table.character_set("utf8mb4").collate("utf8mb4_bin");
        }

        manager.create_table(table.to_owned()).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
