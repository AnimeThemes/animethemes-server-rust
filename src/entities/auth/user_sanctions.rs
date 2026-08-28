use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::auth::{sanction, user};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_sanctions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub sanction_id: Option<u64>,
    pub user_id: Option<u64>,
    pub moderator_id: Option<u64>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
    pub reason: Option<String>,

    #[sea_orm(belongs_to, relation_enum = "User", from = "user_id", to = "id")]
    pub user: BelongsTo<Option<user::Entity>>,

    #[sea_orm(
        belongs_to,
        relation_enum = "Moderator",
        from = "moderator_id",
        to = "id"
    )]
    pub moderator: BelongsTo<Option<user::Entity>>,

    #[sea_orm(belongs_to, from = "sanction_id", to = "id")]
    pub sanction: BelongsTo<Option<sanction::Entity>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn get_forbidden_message(&self) -> String {
        match &self.expires_at {
            Some(expires_at) => {
                format!("until {}", expires_at.format("%Y-%m-%d %H:%M:%S UTC"))
            }
            None => "permanently".to_string(),
        }
    }
}
