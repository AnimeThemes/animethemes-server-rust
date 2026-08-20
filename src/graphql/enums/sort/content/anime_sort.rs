use animethemes_server_rust::entities::content::anime;
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnimeSort {
    Id,
    IdDesc,
    TitleRomaji,
    TitleRomajiDesc,
    TitleEnglish,
    TitleEnglishDesc,
    TitleNative,
    TitleNativeDesc,
    Year,
    YearDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort for AnimeSort {
    type Entity = anime::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (column, direction) = match self {
            Self::Id => (anime::Column::Id, Order::Asc),
            Self::IdDesc => (anime::Column::Id, Order::Desc),

            Self::TitleRomaji => (anime::Column::Title, Order::Asc),
            Self::TitleRomajiDesc => (anime::Column::Title, Order::Desc),

            Self::TitleEnglish => (anime::Column::TitleEnglish, Order::Asc),
            Self::TitleEnglishDesc => (anime::Column::TitleEnglish, Order::Desc),

            Self::TitleNative => (anime::Column::TitleNative, Order::Asc),
            Self::TitleNativeDesc => (anime::Column::TitleNative, Order::Desc),

            Self::Year => (anime::Column::Year, Order::Asc),
            Self::YearDesc => (anime::Column::Year, Order::Desc),

            Self::CreatedAt => (anime::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (anime::Column::CreatedAt, Order::Desc),

            Self::UpdatedAt => (anime::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (anime::Column::UpdatedAt, Order::Desc),

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
