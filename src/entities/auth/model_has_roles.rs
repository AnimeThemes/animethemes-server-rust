use sea_orm::entity::prelude::*;

use crate::entities::auth::{role, user};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "model_has_roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub role_id: u64,
    #[sea_orm(primary_key)]
    pub model_type: String,
    #[sea_orm(primary_key)]
    pub model_id: u64,

    #[sea_orm(belongs_to, from = "model_id", to = "id")]
    pub user: BelongsTo<user::Entity>,

    #[sea_orm(belongs_to, from = "role_id", to = "id")]
    pub role: BelongsTo<role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
