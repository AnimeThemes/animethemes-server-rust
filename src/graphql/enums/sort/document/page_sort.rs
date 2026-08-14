use animethemes_server_rust::entities::document::page;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PageSort {
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

impl GraphQLSort<page::Entity> for PageSort {
    fn apply_sort(&self, query: Select<page::Entity>) -> Select<page::Entity> {
        match self {
            PageSort::Id => query.order_by_asc(page::Column::Id),
            PageSort::IdDesc => query.order_by_desc(page::Column::Id),
            PageSort::Name => query.order_by_asc(page::Column::Name),
            PageSort::NameDesc => query.order_by_desc(page::Column::Name),
            PageSort::CreatedAt => query.order_by_asc(page::Column::CreatedAt),
            PageSort::CreatedAtDesc => query.order_by_desc(page::Column::CreatedAt),
            PageSort::UpdatedAt => query.order_by_asc(page::Column::UpdatedAt),
            PageSort::UpdatedAtDesc => query.order_by_desc(page::Column::UpdatedAt),
            PageSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
