use crate::{entities::content::anime, graphql::cursor::CursorSortKey};
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

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

impl GraphQLSort for AnimeSort {
    type Entity = anime::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (key, order) = match self {
            Self::Id => (CursorSortKey::Column(anime::Column::Id), Order::Asc),
            Self::IdDesc => (CursorSortKey::Column(anime::Column::Id), Order::Desc),
            Self::TitleRomaji => (CursorSortKey::Column(anime::Column::Title), Order::Asc),
            Self::TitleRomajiDesc => (CursorSortKey::Column(anime::Column::Title), Order::Desc),
            Self::TitleEnglish => (
                CursorSortKey::Column(anime::Column::TitleEnglish),
                Order::Asc,
            ),
            Self::TitleEnglishDesc => (
                CursorSortKey::Column(anime::Column::TitleEnglish),
                Order::Desc,
            ),
            Self::TitleNative => (
                CursorSortKey::Column(anime::Column::TitleNative),
                Order::Asc,
            ),
            Self::TitleNativeDesc => (
                CursorSortKey::Column(anime::Column::TitleNative),
                Order::Desc,
            ),
            Self::Year => (CursorSortKey::Column(anime::Column::Year), Order::Asc),
            Self::YearDesc => (CursorSortKey::Column(anime::Column::Year), Order::Desc),
            Self::CreatedAt => (CursorSortKey::Column(anime::Column::CreatedAt), Order::Asc),
            Self::CreatedAtDesc => (CursorSortKey::Column(anime::Column::CreatedAt), Order::Desc),
            Self::UpdatedAt => (CursorSortKey::Column(anime::Column::UpdatedAt), Order::Asc),
            Self::UpdatedAtDesc => (CursorSortKey::Column(anime::Column::UpdatedAt), Order::Desc),
            Self::Random => (
                CursorSortKey::Random {
                    id: anime::Column::Id,
                },
                Order::Asc,
            ),
        };

        Some(CursorSort { key, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(anime::Column::Id),
            Self::IdDesc => query.order_by_desc(anime::Column::Id),
            Self::TitleRomaji => query.order_by_asc(anime::Column::Title),
            Self::TitleRomajiDesc => query.order_by_desc(anime::Column::Title),
            Self::TitleEnglish => query.order_by_asc(anime::Column::TitleEnglish),
            Self::TitleEnglishDesc => query.order_by_desc(anime::Column::TitleEnglish),
            Self::TitleNative => query.order_by_asc(anime::Column::TitleNative),
            Self::TitleNativeDesc => query.order_by_desc(anime::Column::TitleNative),
            Self::Year => query.order_by_asc(anime::Column::Year),
            Self::YearDesc => query.order_by_desc(anime::Column::Year),
            Self::CreatedAt => query.order_by_asc(anime::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(anime::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(anime::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(anime::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
