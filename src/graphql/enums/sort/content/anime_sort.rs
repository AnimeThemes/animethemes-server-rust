use animethemes_server_rust::entities::content::anime;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnimeSort {
    Id,
    IdDesc,
    TitleRomaji,
    TitleRomajiDesc,
    TitleEnglish,
    TitleEnglishDesc,
    TitleNative,
    TitleNativeDesc,
    Year,
    YearDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort<anime::Entity> for AnimeSort {
    fn apply_sort(&self, query: Select<anime::Entity>) -> Select<anime::Entity> {
        match self {
            AnimeSort::Id => query.order_by_asc(anime::Column::Id),
            AnimeSort::IdDesc => query.order_by_desc(anime::Column::Id),
            AnimeSort::TitleRomaji => query.order_by_asc(anime::Column::Title),
            AnimeSort::TitleRomajiDesc => query.order_by_desc(anime::Column::Title),
            AnimeSort::TitleEnglish => query.order_by_asc(anime::Column::TitleEnglish),
            AnimeSort::TitleEnglishDesc => query.order_by_desc(anime::Column::TitleEnglish),
            AnimeSort::TitleNative => query.order_by_asc(anime::Column::TitleNative),
            AnimeSort::TitleNativeDesc => query.order_by_desc(anime::Column::TitleNative),
            AnimeSort::Year => query.order_by_asc(anime::Column::Year),
            AnimeSort::YearDesc => query.order_by_desc(anime::Column::Year),
            AnimeSort::CreatedAt => query.order_by_asc(anime::Column::CreatedAt),
            AnimeSort::CreatedAtDesc => query.order_by_desc(anime::Column::CreatedAt),
            AnimeSort::UpdatedAt => query.order_by_asc(anime::Column::UpdatedAt),
            AnimeSort::UpdatedAtDesc => query.order_by_desc(anime::Column::UpdatedAt),
            AnimeSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
