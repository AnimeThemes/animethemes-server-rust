use animethemes_graphql_rust::entities::list::track;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlaylistTrackSort {
    Position,
    PositionDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort<track::Entity> for PlaylistTrackSort {
    fn apply_sort(&self, query: Select<track::Entity>) -> Select<track::Entity> {
        match self {
            PlaylistTrackSort::Position => query.order_by_asc(track::Column::Position),
            PlaylistTrackSort::PositionDesc => query.order_by_desc(track::Column::Position),
            PlaylistTrackSort::CreatedAt => query.order_by_asc(track::Column::CreatedAt),
            PlaylistTrackSort::CreatedAtDesc => query.order_by_desc(track::Column::CreatedAt),
            PlaylistTrackSort::UpdatedAt => query.order_by_asc(track::Column::UpdatedAt),
            PlaylistTrackSort::UpdatedAtDesc => query.order_by_desc(track::Column::UpdatedAt),
            PlaylistTrackSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
