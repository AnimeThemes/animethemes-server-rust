use crate::entities::list::playlist;
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlaylistSort {
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

impl GraphQLSort for PlaylistSort {
    type Entity = playlist::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (column, order) = match self {
            Self::Id => (playlist::Column::Id, Order::Asc),
            Self::IdDesc => (playlist::Column::Id, Order::Desc),
            Self::Name => (playlist::Column::Name, Order::Asc),
            Self::NameDesc => (playlist::Column::Name, Order::Desc),
            Self::CreatedAt => (playlist::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (playlist::Column::CreatedAt, Order::Desc),
            Self::UpdatedAt => (playlist::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (playlist::Column::UpdatedAt, Order::Desc),
            Self::Random => return None,
        };

        Some(CursorSort { column, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(playlist::Column::Id),
            Self::IdDesc => query.order_by_desc(playlist::Column::Id),
            Self::Name => query.order_by_asc(playlist::Column::Name),
            Self::NameDesc => query.order_by_desc(playlist::Column::Name),
            Self::CreatedAt => query.order_by_asc(playlist::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(playlist::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(playlist::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(playlist::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
