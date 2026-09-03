use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{auth::user, content::animethemeentry};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "favorites")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub favoriteable_type: String,
    pub favoriteable_id: u64,
    pub user_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "favoriteable_id", to = "id")]
    pub favoriteable: BelongsTo<animethemeentry::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
