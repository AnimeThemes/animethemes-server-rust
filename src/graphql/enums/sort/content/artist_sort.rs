use animethemes_server_rust::entities::content::artist;
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

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
        let (column, direction) = match self {
            Self::Id => (artist::Column::Id, Order::Asc),
            Self::IdDesc => (artist::Column::Id, Order::Desc),

            Self::NameMain => (artist::Column::Name, Order::Asc),
            Self::NameMainDesc => (artist::Column::Name, Order::Desc),

            Self::NameNative => (artist::Column::NameNative, Order::Asc),
            Self::NameNativeDesc => (artist::Column::NameNative, Order::Desc),

            Self::CreatedAt => (artist::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (artist::Column::CreatedAt, Order::Desc),

            Self::UpdatedAt => (artist::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (artist::Column::UpdatedAt, Order::Desc),

            Self::Random => return None,
        };

        Some(CursorSort {
            column,
            order: direction,
        })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        let cursor_sort = self.cursor_sort();

        match cursor_sort {
            Some(cursor_sort) => query.order_by(cursor_sort.column, cursor_sort.order),
            None => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
