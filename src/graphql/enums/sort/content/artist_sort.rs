use animethemes_graphql_rust::entities::content::artist;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ArtistSort {
    Id,
    IdDesc,
    NameMain,
    NameMainDesc,
    NameNative,
    NameNativeDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort<artist::Entity> for ArtistSort {
    fn apply_sort(&self, query: Select<artist::Entity>) -> Select<artist::Entity> {
        match self {
            ArtistSort::Id => query.order_by_asc(artist::Column::Id),
            ArtistSort::IdDesc => query.order_by_desc(artist::Column::Id),
            ArtistSort::NameMain => query.order_by_asc(artist::Column::Name),
            ArtistSort::NameMainDesc => query.order_by_desc(artist::Column::Name),
            ArtistSort::NameNative => query.order_by_asc(artist::Column::NameNative),
            ArtistSort::NameNativeDesc => query.order_by_desc(artist::Column::NameNative),
            ArtistSort::CreatedAt => query.order_by_asc(artist::Column::CreatedAt),
            ArtistSort::CreatedAtDesc => query.order_by_desc(artist::Column::CreatedAt),
            ArtistSort::UpdatedAt => query.order_by_asc(artist::Column::UpdatedAt),
            ArtistSort::UpdatedAtDesc => query.order_by_desc(artist::Column::UpdatedAt),
            ArtistSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
