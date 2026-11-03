use crate::entities::content::image;
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ImageSort {
    Id,
    IdDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort for ImageSort {
    type Entity = image::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (column, order) = match self {
            Self::Id => (image::Column::Id, Order::Asc),
            Self::IdDesc => (image::Column::Id, Order::Desc),
            Self::CreatedAt => (image::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (image::Column::CreatedAt, Order::Desc),
            Self::UpdatedAt => (image::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (image::Column::UpdatedAt, Order::Desc),
            Self::Random => return None,
        };

        Some(CursorSort { column, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(image::Column::Id),
            Self::IdDesc => query.order_by_desc(image::Column::Id),
            Self::CreatedAt => query.order_by_asc(image::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(image::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(image::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(image::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
