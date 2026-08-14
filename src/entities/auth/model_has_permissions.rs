use sea_orm::entity::prelude::*;

use crate::entities::auth::{permission, user};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "model_has_permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub permission_id: u64,
    #[sea_orm(primary_key)]
    pub model_type: String,
    #[sea_orm(primary_key)]
    pub model_id: u64,

    #[sea_orm(belongs_to, from = "model_id", to = "id")]
    pub user: BelongsTo<user::Entity>,

    #[sea_orm(belongs_to, from = "permission_id", to = "id")]
    pub permission: BelongsTo<permission::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
