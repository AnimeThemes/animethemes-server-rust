use std::{fmt, str::FromStr};

use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::entities::auth::{user, user_sanctions};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sanctions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    #[sea_orm(unique)]
    pub name: String,
    #[serde(default)]
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(default)]
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
}

pub struct SanctionToUser;

impl Linked for SanctionToUser {
    type FromEntity = Entity;
    type ToEntity = user::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            user_sanctions::Relation::Sanction.def().rev(),
            user_sanctions::Relation::User.def(),
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(PartialEq, Eq)]
pub enum Sanctions {
    PlaylistManagement,
}

impl Sanctions {
    pub fn get_forbidden_message(&self, message: String) -> String {
        match self {
            Sanctions::PlaylistManagement => {
                format!("You are not allowed to manage playlists {}.", message)
            }
        }
    }
}

impl FromStr for Sanctions {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "playlist management" => Ok(Sanctions::PlaylistManagement),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Sanctions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Sanctions::PlaylistManagement => "playlist management",
        };

        f.write_str(value)
    }
}
