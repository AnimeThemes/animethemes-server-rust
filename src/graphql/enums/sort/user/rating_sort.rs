use crate::{entities::user::rating, graphql::cursor::CursorSortKey};
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum RatingSort {
    Score,
    ScoreDesc,
    Random,
}

impl GraphQLSort for RatingSort {
    type Entity = rating::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (key, order) = match self {
            Self::Score => (CursorSortKey::Column(rating::Column::Score), Order::Asc),
            Self::ScoreDesc => (CursorSortKey::Column(rating::Column::Score), Order::Desc),
            Self::Random => (
                CursorSortKey::Random {
                    id: rating::Column::Id,
                },
                Order::Asc,
            ),
        };

        Some(CursorSort { key, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Score => query.order_by_asc(rating::Column::Score),
            Self::ScoreDesc => query.order_by_desc(rating::Column::Score),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
