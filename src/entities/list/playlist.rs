use chrono::Utc;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

use async_trait::async_trait;

use crate::{
    entities::{HasHashId, auth::user, list::track},
    enums::list::playlistvisibility::PlaylistVisibility,
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "playlists")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "playlist_id")]
    pub id: u64,
    #[sea_orm(unique)]
    pub hashid: Option<String>,
    pub description: Option<String>,
    pub name: String,
    pub user_id: u64,
    pub visibility: PlaylistVisibility,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<user::Entity>,

    #[sea_orm(has_many, relation_enum = "Tracks")]
    pub tracks: HasMany<track::Entity>,
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

impl HasHashId for Model {
    fn hashids(&self) -> Vec<u64> {
        vec![self.user_id, self.id]
    }
}
