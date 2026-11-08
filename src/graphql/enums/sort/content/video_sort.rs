use crate::{entities::content::video, graphql::cursor::CursorSortKey};
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum VideoSort {
    Id,
    IdDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort for VideoSort {
    type Entity = video::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (key, order) = match self {
            Self::Id => (CursorSortKey::Column(video::Column::Id), Order::Asc),
            Self::IdDesc => (CursorSortKey::Column(video::Column::Id), Order::Desc),
            Self::CreatedAt => (CursorSortKey::Column(video::Column::CreatedAt), Order::Asc),
            Self::CreatedAtDesc => (CursorSortKey::Column(video::Column::CreatedAt), Order::Desc),
            Self::UpdatedAt => (CursorSortKey::Column(video::Column::UpdatedAt), Order::Asc),
            Self::UpdatedAtDesc => (CursorSortKey::Column(video::Column::UpdatedAt), Order::Desc),
            Self::Random => (
                CursorSortKey::Random {
                    id: video::Column::Id,
                },
                Order::Asc,
            ),
        };

        Some(CursorSort { key, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(video::Column::Id),
            Self::IdDesc => query.order_by_desc(video::Column::Id),
            Self::CreatedAt => query.order_by_asc(video::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(video::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(video::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(video::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
