use sea_orm::entity::prelude::*;

use crate::entities::auth::{role, user};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub role_id: u64,
    #[sea_orm(primary_key)]
    pub user_id: u64,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<user::Entity>,

    #[sea_orm(belongs_to, from = "role_id", to = "id")]
    pub role: BelongsTo<role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
