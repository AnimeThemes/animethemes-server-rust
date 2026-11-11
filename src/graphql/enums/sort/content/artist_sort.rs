use async_graphql::Enum;
use sea_orm::sea_query::Expr;
use sea_orm::{EntityTrait, Order, QueryOrder, Select};

use crate::entities::content::artist;
use crate::graphql::cursor::{CursorSort, CursorSortKey};
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

impl GraphQLSort for ArtistSort {
    type Entity = artist::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (key, order) = match self {
            Self::Id => (CursorSortKey::Column(artist::Column::Id), Order::Asc),
            Self::IdDesc => (CursorSortKey::Column(artist::Column::Id), Order::Desc),
            Self::NameMain => (CursorSortKey::Column(artist::Column::Name), Order::Asc),
            Self::NameMainDesc => (CursorSortKey::Column(artist::Column::Name), Order::Desc),
            Self::NameNative => (
                CursorSortKey::Column(artist::Column::NameNative),
                Order::Asc,
            ),
            Self::NameNativeDesc => (
                CursorSortKey::Column(artist::Column::NameNative),
                Order::Desc,
            ),
            Self::CreatedAt => (CursorSortKey::Column(artist::Column::CreatedAt), Order::Asc),
            Self::CreatedAtDesc => (
                CursorSortKey::Column(artist::Column::CreatedAt),
                Order::Desc,
            ),
            Self::UpdatedAt => (CursorSortKey::Column(artist::Column::UpdatedAt), Order::Asc),
            Self::UpdatedAtDesc => (
                CursorSortKey::Column(artist::Column::UpdatedAt),
                Order::Desc,
            ),
            Self::Random => (
                CursorSortKey::Random {
                    id: artist::Column::Id,
                },
                Order::Asc,
            ),
        };

        Some(CursorSort { key, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(artist::Column::Id),
            Self::IdDesc => query.order_by_desc(artist::Column::Id),
            Self::NameMain => query.order_by_asc(artist::Column::Name),
            Self::NameMainDesc => query.order_by_desc(artist::Column::Name),
            Self::NameNative => query.order_by_asc(artist::Column::NameNative),
            Self::NameNativeDesc => query.order_by_desc(artist::Column::NameNative),
            Self::CreatedAt => query.order_by_asc(artist::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(artist::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(artist::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(artist::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
