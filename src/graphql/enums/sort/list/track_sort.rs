use crate::{entities::list::track, graphql::cursor::CursorSortKey};
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlaylistTrackSort {
    Position,
    PositionDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort for PlaylistTrackSort {
    type Entity = track::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (key, order) = match self {
            Self::Position => (CursorSortKey::Column(track::Column::Position), Order::Asc),
            Self::PositionDesc => (CursorSortKey::Column(track::Column::Position), Order::Desc),
            Self::CreatedAt => (CursorSortKey::Column(track::Column::CreatedAt), Order::Asc),
            Self::CreatedAtDesc => (CursorSortKey::Column(track::Column::CreatedAt), Order::Desc),
            Self::UpdatedAt => (CursorSortKey::Column(track::Column::UpdatedAt), Order::Asc),
            Self::UpdatedAtDesc => (CursorSortKey::Column(track::Column::UpdatedAt), Order::Desc),
            Self::Random => (
                CursorSortKey::Random {
                    id: track::Column::Id,
                },
                Order::Asc,
            ),
        };

        Some(CursorSort { key, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Position => query.order_by_asc(track::Column::Position),
            Self::PositionDesc => query.order_by_desc(track::Column::Position),
            Self::CreatedAt => query.order_by_asc(track::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(track::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(track::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(track::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
