use animethemes_graphql_rust::entities::list::playlist;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
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

impl GraphQLSort<playlist::Entity> for PlaylistSort {
    fn apply_sort(&self, query: Select<playlist::Entity>) -> Select<playlist::Entity> {
        match self {
            PlaylistSort::Id => query.order_by_asc(playlist::Column::Id),
            PlaylistSort::IdDesc => query.order_by_desc(playlist::Column::Id),
            PlaylistSort::Name => query.order_by_asc(playlist::Column::Name),
            PlaylistSort::NameDesc => query.order_by_desc(playlist::Column::Name),
            PlaylistSort::CreatedAt => query.order_by_asc(playlist::Column::CreatedAt),
            PlaylistSort::CreatedAtDesc => query.order_by_desc(playlist::Column::CreatedAt),
            PlaylistSort::UpdatedAt => query.order_by_asc(playlist::Column::UpdatedAt),
            PlaylistSort::UpdatedAtDesc => query.order_by_desc(playlist::Column::UpdatedAt),
            PlaylistSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
