use animethemes_server_rust::entities::document::page;
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

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

impl GraphQLSort for PageSort {
    type Entity = page::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (column, direction) = match self {
            Self::Id => (page::Column::Id, Order::Asc),
            Self::IdDesc => (page::Column::Id, Order::Desc),

            Self::Name => (page::Column::Name, Order::Asc),
            Self::NameDesc => (page::Column::Name, Order::Desc),

            Self::CreatedAt => (page::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (page::Column::CreatedAt, Order::Desc),

            Self::UpdatedAt => (page::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (page::Column::UpdatedAt, Order::Desc),

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
