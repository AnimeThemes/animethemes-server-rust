use animethemes_graphql_rust::entities::content::image;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

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

impl GraphQLSort<image::Entity> for ImageSort {
    fn apply_sort(&self, query: Select<image::Entity>) -> Select<image::Entity> {
        match self {
            ImageSort::Id => query.order_by_asc(image::Column::Id),
            ImageSort::IdDesc => query.order_by_desc(image::Column::Id),
            ImageSort::CreatedAt => query.order_by_asc(image::Column::CreatedAt),
            ImageSort::CreatedAtDesc => query.order_by_desc(image::Column::CreatedAt),
            ImageSort::UpdatedAt => query.order_by_asc(image::Column::UpdatedAt),
            ImageSort::UpdatedAtDesc => query.order_by_desc(image::Column::UpdatedAt),
            ImageSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
