use sea_orm::{ColumnTrait, Condition};

use crate::entities::content::anime;

pub fn anime_by_title_like(title: String) -> Condition {
    Condition::all().add(anime::Column::Title.like(title))
}

pub fn anime_by_year(year: i16) -> Condition {
    Condition::all().add(anime::Column::Year.eq(year))
}
