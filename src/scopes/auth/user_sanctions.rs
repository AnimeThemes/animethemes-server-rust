use sea_orm::{ColumnTrait, Condition};

use crate::entities::auth::user_sanctions;

pub fn current_sanctions() -> Condition {
    Condition::any()
        .add(user_sanctions::Column::ExpiresAt.is_null())
        .add(user_sanctions::Column::ExpiresAt.gt(time::OffsetDateTime::now_utc()))
}
