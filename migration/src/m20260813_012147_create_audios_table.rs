use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260813_012147_create_audios_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("audios")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("audio_id")
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("basename").string().not_null())
                    .col(ColumnDef::new("filename").string().not_null())
                    .col(ColumnDef::new("path").string().not_null())
                    .col(ColumnDef::new("size").integer().not_null())
                    .col(ColumnDef::new("mimetype").string().not_null())
                    .col(ColumnDef::new("created_at").timestamp().null())
                    .col(ColumnDef::new("updated_at").timestamp().null())
                    .col(ColumnDef::new("deleted_at").timestamp().null())
                    .to_owned(),
            )
            .await?;

        if !manager.has_column("videos", "audio_id").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table("videos")
                        .add_column(ColumnDef::new("audio_id").big_unsigned().null())
                        .add_foreign_key(
                            TableForeignKey::new()
                                .name("videos_audio_id_foreign")
                                .from_tbl("videos")
                                .from_col("audio_id")
                                .to_tbl("audios")
                                .to_col("audio_id")
                                .on_delete(ForeignKeyAction::SetNull),
                        )
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
