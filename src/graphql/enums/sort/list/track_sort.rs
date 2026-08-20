use animethemes_server_rust::entities::list::track;
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
        let (column, direction) = match self {
            Self::Position => (track::Column::Position, Order::Asc),
            Self::PositionDesc => (track::Column::Position, Order::Desc),

            Self::CreatedAt => (track::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (track::Column::CreatedAt, Order::Desc),

            Self::UpdatedAt => (track::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (track::Column::UpdatedAt, Order::Desc),

            Self::Random => return None,
        };

        Some(CursorSort {
            column,
            order: direction,
        })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        let cursor_sort = self.cursor_sort();

        match cursor_sort {
            Some(cursor_sort) => query.order_by(cursor_sort.column, cursor_sort.order),
            None => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
