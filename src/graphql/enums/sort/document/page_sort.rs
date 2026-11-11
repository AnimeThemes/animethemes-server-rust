use async_graphql::Enum;
use sea_orm::sea_query::Expr;
use sea_orm::{EntityTrait, Order, QueryOrder, Select};

use crate::entities::document::page;
use crate::graphql::cursor::{CursorSort, CursorSortKey};
use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PageSort {
    Id,
    IdDesc,
    Name,
    NameDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort for PageSort {
    type Entity = page::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (key, order) = match self {
            Self::Id => (CursorSortKey::Column(page::Column::Id), Order::Asc),
            Self::IdDesc => (CursorSortKey::Column(page::Column::Id), Order::Desc),
            Self::Name => (CursorSortKey::Column(page::Column::Name), Order::Asc),
            Self::NameDesc => (CursorSortKey::Column(page::Column::Name), Order::Desc),
            Self::CreatedAt => (CursorSortKey::Column(page::Column::CreatedAt), Order::Asc),
            Self::CreatedAtDesc => (CursorSortKey::Column(page::Column::CreatedAt), Order::Desc),
            Self::UpdatedAt => (CursorSortKey::Column(page::Column::UpdatedAt), Order::Asc),
            Self::UpdatedAtDesc => (CursorSortKey::Column(page::Column::UpdatedAt), Order::Desc),
            Self::Random => (
                CursorSortKey::Random {
                    id: page::Column::Id,
                },
                Order::Asc,
            ),
        };

        Some(CursorSort { key, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(page::Column::Id),
            Self::IdDesc => query.order_by_desc(page::Column::Id),
            Self::Name => query.order_by_asc(page::Column::Name),
            Self::NameDesc => query.order_by_desc(page::Column::Name),
            Self::CreatedAt => query.order_by_asc(page::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(page::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(page::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(page::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
