use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::entities::{auth::role, list::playlist};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    #[sea_orm(column_type = "Timestamp")]
    pub email_verified_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(has_many, via = "user_roles")]
    pub roles: HasMany<role::Entity>,

    #[sea_orm(has_many)]
    pub playlists: HasMany<playlist::Entity>,
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
}
