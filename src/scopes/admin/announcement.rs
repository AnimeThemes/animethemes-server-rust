use sea_orm::{ColumnTrait, Condition};

use crate::entities::admin::announcement;

pub fn current_announcement() -> Condition {
    Condition::all()
        .add(announcement::Column::StartAt.lte(time::OffsetDateTime::now_utc()))
        .add(announcement::Column::EndAt.gte(time::OffsetDateTime::now_utc()))
}
