use crate::entities::user::rating;
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
        let (column, direction) = match self {
            Self::Score => (rating::Column::Score, Order::Asc),
            Self::ScoreDesc => (rating::Column::Score, Order::Desc),
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
