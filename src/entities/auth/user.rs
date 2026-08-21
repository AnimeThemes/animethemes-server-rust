use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    auth::{permission, role},
    list::playlist,
};

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

    #[sea_orm(has_many, via = "model_has_roles")]
    pub roles: HasMany<role::Entity>,

    #[sea_orm(has_many, via = "model_has_permissions")]
    pub permissions: HasMany<permission::Entity>,

    #[sea_orm(has_many)]
    pub playlists: HasMany<playlist::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
