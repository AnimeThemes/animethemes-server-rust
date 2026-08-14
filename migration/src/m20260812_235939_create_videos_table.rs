use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260812_235939_create_videos_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("videos")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("video_id")
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
                    .col(ColumnDef::new("resolution").integer().null())
                    .col(ColumnDef::new("nc").boolean().default(false))
                    .col(ColumnDef::new("subbed").boolean().default(false))
                    .col(ColumnDef::new("lyrics").boolean().default(false))
                    .col(ColumnDef::new("uncen").boolean().default(false))
                    .col(ColumnDef::new("overlap").integer().not_null())
                    .col(ColumnDef::new("source").integer().null())
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
