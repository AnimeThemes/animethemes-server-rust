use async_graphql::Enum;
use sea_orm::sea_query::Expr;
use sea_orm::{EntityTrait, Order, QueryOrder, Select};

use crate::entities::content::series;
use crate::graphql::cursor::{CursorSort, CursorSortKey};
use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SeriesSort {
    Id,
    IdDesc,
    TitleRomaji,
    TitleRomajiDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort for SeriesSort {
    type Entity = series::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (key, order) = match self {
            Self::Id => (CursorSortKey::Column(series::Column::Id), Order::Asc),
            Self::IdDesc => (CursorSortKey::Column(series::Column::Id), Order::Desc),
            Self::TitleRomaji => (CursorSortKey::Column(series::Column::Title), Order::Asc),
            Self::TitleRomajiDesc => (CursorSortKey::Column(series::Column::Title), Order::Desc),
            Self::CreatedAt => (CursorSortKey::Column(series::Column::CreatedAt), Order::Asc),
            Self::CreatedAtDesc => (
                CursorSortKey::Column(series::Column::CreatedAt),
                Order::Desc,
            ),
            Self::UpdatedAt => (CursorSortKey::Column(series::Column::UpdatedAt), Order::Asc),
            Self::UpdatedAtDesc => (
                CursorSortKey::Column(series::Column::UpdatedAt),
                Order::Desc,
            ),
            Self::Random => (
                CursorSortKey::Random {
                    id: series::Column::Id,
                },
                Order::Asc,
            ),
        };

        Some(CursorSort { key, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(series::Column::Id),
            Self::IdDesc => query.order_by_desc(series::Column::Id),
            Self::TitleRomaji => query.order_by_asc(series::Column::Title),
            Self::TitleRomajiDesc => query.order_by_desc(series::Column::Title),
            Self::CreatedAt => query.order_by_asc(series::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(series::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(series::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(series::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
