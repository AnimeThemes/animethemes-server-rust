use sea_orm::sea_query::IntoCondition;
use sea_orm::{ColumnTrait, Condition};

use crate::traits::soft_delete::SoftDeleteEntity;

pub mod admin;
pub mod auth;
pub mod document;
pub mod list;

pub fn without_trashed<E: SoftDeleteEntity>() -> Condition {
    E::deleted_at_column().is_null().into_condition()
}
