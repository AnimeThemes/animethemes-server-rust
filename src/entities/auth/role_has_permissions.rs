use sea_orm::entity::prelude::*;

use crate::entities::auth::{permission, role};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "role_has_permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub role_id: u64,
    #[sea_orm(primary_key)]
    pub permission_id: u64,

    #[sea_orm(belongs_to, from = "role_id", to = "id")]
    pub role: BelongsTo<role::Entity>,

    #[sea_orm(belongs_to, from = "permission_id", to = "id")]
    pub permission: BelongsTo<permission::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
