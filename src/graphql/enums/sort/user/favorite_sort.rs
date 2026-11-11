use async_graphql::Enum;
use sea_orm::sea_query::Expr;
use sea_orm::{EntityTrait, Order, QueryOrder, Select};

use crate::entities::user::favorite;
use crate::graphql::cursor::{CursorSort, CursorSortKey};
use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum FavoriteSort {
    Id,
    IdDesc,
    Position,
    PositionDesc,
    Random,
}

impl GraphQLSort for FavoriteSort {
    type Entity = favorite::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (key, order) = match self {
            Self::Id => (CursorSortKey::Column(favorite::Column::Id), Order::Asc),
            Self::IdDesc => (CursorSortKey::Column(favorite::Column::Id), Order::Desc),
            Self::Position => (
                CursorSortKey::Column(favorite::Column::Position),
                Order::Asc,
            ),
            Self::PositionDesc => (
                CursorSortKey::Column(favorite::Column::Position),
                Order::Desc,
            ),
            Self::Random => (
                CursorSortKey::Random {
                    id: favorite::Column::Id,
                },
                Order::Asc,
            ),
        };

        Some(CursorSort { key, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(favorite::Column::Id),
            Self::IdDesc => query.order_by_desc(favorite::Column::Id),
            Self::Position => query.order_by_asc(favorite::Column::Position),
            Self::PositionDesc => query.order_by_desc(favorite::Column::Position),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
