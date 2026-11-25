use sea_orm::{ColumnTrait, Condition};

use crate::entities::admin::featuredtheme;

pub fn current_featured_theme() -> Condition {
    Condition::all()
        .add(featuredtheme::Column::StartAt.lte(time::OffsetDateTime::now_utc()))
        .add(featuredtheme::Column::EndAt.gte(time::OffsetDateTime::now_utc()))
}
