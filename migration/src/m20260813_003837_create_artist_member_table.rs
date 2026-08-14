use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_003837_create_artist_member_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("artist_member")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("artist_id").big_unsigned().not_null())
                    .col(ColumnDef::new("member_id").big_unsigned().not_null())
                    .col(ColumnDef::new("relevance").integer().default(1).null())
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("artist_member_artist_id_foreign")
                            .from("artist_member", "artist_id")
                            .to("artists", "artist_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("artist_member_member_id_foreign")
                            .from("artist_member", "member_id")
                            .to("artists", "artist_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("artist_member_unique_index")
                            .col("artist_id")
                            .col("member_id")
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
