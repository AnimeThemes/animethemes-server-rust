use std::{fmt, str::FromStr};

use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::auth::{permission, user};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    #[sea_orm(unique)]
    pub name: String,
    pub priority: i32,
    pub color: Option<String>,
    pub default: bool,
    pub guard_name: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(has_many, via = "role_has_permissions")]
    pub permissions: HasMany<permission::Entity>,

    #[sea_orm(has_many, via = "model_has_roles")]
    pub users: HasMany<user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(PartialEq, Eq)]
pub enum Roles {
    Admin,
    Encoder,
    Developer,
    ContentModerator,
    Patron,
    Contributor,
    Verified,
}

impl FromStr for Roles {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Admin" => Ok(Roles::Admin),
            "Encoder" => Ok(Roles::Encoder),
            "Developer" => Ok(Roles::Developer),
            "Content Moderator" => Ok(Roles::ContentModerator),
            "Patron" => Ok(Roles::Patron),
            "Contributor" => Ok(Roles::Contributor),
            "Verified" => Ok(Roles::Verified),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Roles::Admin => "Admin",
            Roles::Encoder => "Encoder",
            Roles::Developer => "Developer",
            Roles::ContentModerator => "Content Moderator",
            Roles::Patron => "Patron",
            Roles::Contributor => "Contributor",
            Roles::Verified => "Verified",
        };

        f.write_str(value)
    }
}
