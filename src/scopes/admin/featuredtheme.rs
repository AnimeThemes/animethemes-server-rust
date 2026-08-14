use sea_orm::{ColumnTrait, Condition};

use crate::entities::admin::featuredtheme;

pub fn current_featured_theme() -> Condition {
    Condition::all()
        .add(featuredtheme::Column::StartAt.lte(chrono::Utc::now()))
        .add(featuredtheme::Column::EndAt.gte(chrono::Utc::now()))
}
