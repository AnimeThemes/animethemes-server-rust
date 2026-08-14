use sea_orm::{ColumnTrait, Condition, sea_query::IntoCondition};

use crate::entities::SoftDeleteEntity;

pub mod admin;
pub mod content;
pub mod list;

pub fn without_trashed<E: SoftDeleteEntity>() -> Condition {
    E::deleted_at_column().is_null().into_condition()
}
