use crate::entities::content::video;
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
        let (column, order) = match self {
            Self::Id => (video::Column::Id, Order::Asc),
            Self::IdDesc => (video::Column::Id, Order::Desc),
            Self::CreatedAt => (video::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (video::Column::CreatedAt, Order::Desc),
            Self::UpdatedAt => (video::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (video::Column::UpdatedAt, Order::Desc),
            Self::Random => return None,
        };

        Some(CursorSort { column, order })
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
