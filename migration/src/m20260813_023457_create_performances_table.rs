use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_023457_create_performances_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("performances")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("performance_id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("song_id").big_unsigned().not_null())
                    .col(ColumnDef::new("artist_id").big_unsigned().not_null())
                    .col(ColumnDef::new("member_id").big_unsigned().null())
                    .col(ColumnDef::new("alias").string().null())
                    .col(ColumnDef::new("as").string().null())
                    .col(ColumnDef::new("member_alias").string().null())
                    .col(ColumnDef::new("member_as").string().null())
                    .col(ColumnDef::new("relevance").integer().not_null().default(1))
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
                    .col(ColumnDef::new("deleted_at").timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("performances_song_id_foreign")
                            .from("performances", "song_id")
                            .to("songs", "song_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("performances_artist_id_foreign")
                            .from("performances", "artist_id")
                            .to("artists", "artist_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("performances_member_id_foreign")
                            .from("performances", "member_id")
                            .to("artists", "artist_id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .index(
                        Index::create()
                            .name("unique_performance")
                            .col("song_id")
                            .col("artist_id")
                            .col("member_id")
                            .col("deleted_at")
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
