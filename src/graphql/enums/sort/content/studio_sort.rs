use crate::entities::content::studio;
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum StudioSort {
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

impl GraphQLSort for StudioSort {
    type Entity = studio::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (column, order) = match self {
            Self::Id => (studio::Column::Id, Order::Asc),
            Self::IdDesc => (studio::Column::Id, Order::Desc),
            Self::Name => (studio::Column::Name, Order::Asc),
            Self::NameDesc => (studio::Column::Name, Order::Desc),
            Self::CreatedAt => (studio::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (studio::Column::CreatedAt, Order::Desc),
            Self::UpdatedAt => (studio::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (studio::Column::UpdatedAt, Order::Desc),
            Self::Random => return None,
        };

        Some(CursorSort { column, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(studio::Column::Id),
            Self::IdDesc => query.order_by_desc(studio::Column::Id),
            Self::Name => query.order_by_asc(studio::Column::Name),
            Self::NameDesc => query.order_by_desc(studio::Column::Name),
            Self::CreatedAt => query.order_by_asc(studio::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(studio::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(studio::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(studio::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
