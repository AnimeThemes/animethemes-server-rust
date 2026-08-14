use sea_orm::{ColumnTrait, Condition};

use crate::entities::admin::announcement;

pub fn current_announcement() -> Condition {
    Condition::all()
        .add(announcement::Column::StartAt.lte(chrono::Utc::now()))
        .add(announcement::Column::EndAt.gte(chrono::Utc::now()))
}
