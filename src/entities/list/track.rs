use chrono::Utc;
use sea_orm::{ActiveValue::Set, Condition, entity::prelude::*};

use async_trait::async_trait;

use crate::{
    entities::{
        HasHashId,
        content::{entry, video},
        list::playlist,
    },
    traits::sortable::Sortable,
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "playlist_tracks")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "track_id")]
    pub id: u64,
    pub hashid: Option<String>,
    pub entry_id: Option<u64>,
    pub playlist_id: u64,
    pub position: i32,
    pub video_id: Option<u64>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "playlist_id", to = "id")]
    pub playlist: BelongsTo<playlist::Entity>,

    #[sea_orm(belongs_to, from = "entry_id", to = "id")]
    pub entry: BelongsTo<Option<entry::Entity>>,

    #[sea_orm(belongs_to, from = "video_id", to = "id")]
    pub video: BelongsTo<Option<video::Entity>>,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut model = self;

        model.updated_at = Set(Utc::now());

        Ok(model)
    }

    async fn after_save<C>(mut model: Model, db: &C, insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            let hashid = model.encode_hashid();

            Entity::update_many()
                .col_expr(Column::Hashid, Expr::value(hashid.clone()))
                .filter(Column::Id.eq(model.id))
                .exec(db)
                .await?;

            model.hashid = Some(hashid);
        }

        Ok(model)
    }
}

impl Sortable for Model {
    fn order_column() -> <Self::Entity as EntityTrait>::Column {
        self::Column::Position
    }

    fn sort_scope(&self) -> sea_orm::Condition {
        Condition::all().add(self::Column::PlaylistId.eq(self.playlist_id))
    }
}

impl HasHashId for Model {
    fn hashids(&self) -> Vec<u64> {
        vec![self.playlist_id, self.id]
    }
}
