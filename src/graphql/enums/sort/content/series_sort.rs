use animethemes_server_rust::entities::content::series;
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SeriesSort {
    Id,
    IdDesc,
    TitleRomaji,
    TitleRomajiDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort for SeriesSort {
    type Entity = series::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (column, direction) = match self {
            Self::Id => (series::Column::Id, Order::Asc),
            Self::IdDesc => (series::Column::Id, Order::Desc),

            Self::TitleRomaji => (series::Column::Title, Order::Asc),
            Self::TitleRomajiDesc => (series::Column::Title, Order::Desc),

            Self::CreatedAt => (series::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (series::Column::CreatedAt, Order::Desc),

            Self::UpdatedAt => (series::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (series::Column::UpdatedAt, Order::Desc),

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
