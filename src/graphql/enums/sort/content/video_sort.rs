use animethemes_server_rust::entities::content::video;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

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

impl GraphQLSort<video::Entity> for VideoSort {
    fn apply_sort(&self, query: Select<video::Entity>) -> Select<video::Entity> {
        match self {
            VideoSort::Id => query.order_by_asc(video::Column::Id),
            VideoSort::IdDesc => query.order_by_desc(video::Column::Id),
            VideoSort::CreatedAt => query.order_by_asc(video::Column::CreatedAt),
            VideoSort::CreatedAtDesc => query.order_by_desc(video::Column::CreatedAt),
            VideoSort::UpdatedAt => query.order_by_asc(video::Column::UpdatedAt),
            VideoSort::UpdatedAtDesc => query.order_by_desc(video::Column::UpdatedAt),
            VideoSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
