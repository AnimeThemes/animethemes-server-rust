use animethemes_server_rust::entities::content::studio;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

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

impl GraphQLSort<studio::Entity> for StudioSort {
    fn apply_sort(&self, query: Select<studio::Entity>) -> Select<studio::Entity> {
        match self {
            StudioSort::Id => query.order_by_asc(studio::Column::Id),
            StudioSort::IdDesc => query.order_by_desc(studio::Column::Id),
            StudioSort::Name => query.order_by_asc(studio::Column::Name),
            StudioSort::NameDesc => query.order_by_desc(studio::Column::Name),
            StudioSort::CreatedAt => query.order_by_asc(studio::Column::CreatedAt),
            StudioSort::CreatedAtDesc => query.order_by_desc(studio::Column::CreatedAt),
            StudioSort::UpdatedAt => query.order_by_asc(studio::Column::UpdatedAt),
            StudioSort::UpdatedAtDesc => query.order_by_desc(studio::Column::UpdatedAt),
            StudioSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
