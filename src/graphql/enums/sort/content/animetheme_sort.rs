use crate::entities::content::animetheme;
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnimeThemeSort {
    Id,
    IdDesc,
    Sequence,
    SequenceDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    Random,
}

impl GraphQLSort for AnimeThemeSort {
    type Entity = animetheme::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (column, direction) = match self {
            Self::Id => (animetheme::Column::Id, Order::Asc),
            Self::IdDesc => (animetheme::Column::Id, Order::Desc),

            Self::Sequence => (animetheme::Column::Sequence, Order::Asc),
            Self::SequenceDesc => (animetheme::Column::Sequence, Order::Desc),

            Self::CreatedAt => (animetheme::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (animetheme::Column::CreatedAt, Order::Desc),

            Self::UpdatedAt => (animetheme::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (animetheme::Column::UpdatedAt, Order::Desc),

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
