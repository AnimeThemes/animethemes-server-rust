use async_graphql::Enum;
use sea_orm::sea_query::Expr;
use sea_orm::{EntityTrait, Order, QueryOrder, Select};

use crate::entities::user::watchhistory;
use crate::graphql::cursor::{CursorSort, CursorSortKey};
use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum WatchHistorySort {
    Id,
    IdDesc,
    Random,
}

impl GraphQLSort for WatchHistorySort {
    type Entity = watchhistory::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (key, order) = match self {
            Self::Id => (CursorSortKey::Column(watchhistory::Column::Id), Order::Asc),
            Self::IdDesc => (CursorSortKey::Column(watchhistory::Column::Id), Order::Desc),
            Self::Random => (
                CursorSortKey::Random {
                    id: watchhistory::Column::Id,
                },
                Order::Asc,
            ),
        };

        Some(CursorSort { key, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(watchhistory::Column::Id),
            Self::IdDesc => query.order_by_desc(watchhistory::Column::Id),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
