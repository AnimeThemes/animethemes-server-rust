use crate::entities::content::theme;
use async_graphql::Enum;
use sea_orm::{EntityTrait, Order, QueryOrder, Select, sea_query::Expr};

use crate::graphql::{cursor::CursorSort, enums::sort::GraphQLSort};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ThemeSort {
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

impl GraphQLSort for ThemeSort {
    type Entity = theme::Entity;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>> {
        let (column, order) = match self {
            Self::Id => (theme::Column::Id, Order::Asc),
            Self::IdDesc => (theme::Column::Id, Order::Desc),
            Self::Sequence => (theme::Column::Sequence, Order::Asc),
            Self::SequenceDesc => (theme::Column::Sequence, Order::Desc),
            Self::CreatedAt => (theme::Column::CreatedAt, Order::Asc),
            Self::CreatedAtDesc => (theme::Column::CreatedAt, Order::Desc),
            Self::UpdatedAt => (theme::Column::UpdatedAt, Order::Asc),
            Self::UpdatedAtDesc => (theme::Column::UpdatedAt, Order::Desc),
            Self::Random => return None,
        };

        Some(CursorSort { column, order })
    }

    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity> {
        match self {
            Self::Id => query.order_by_asc(theme::Column::Id),
            Self::IdDesc => query.order_by_desc(theme::Column::Id),
            Self::Sequence => query.order_by_asc(theme::Column::Sequence),
            Self::SequenceDesc => query.order_by_desc(theme::Column::Sequence),
            Self::CreatedAt => query.order_by_asc(theme::Column::CreatedAt),
            Self::CreatedAtDesc => query.order_by_desc(theme::Column::CreatedAt),
            Self::UpdatedAt => query.order_by_asc(theme::Column::UpdatedAt),
            Self::UpdatedAtDesc => query.order_by_desc(theme::Column::UpdatedAt),
            Self::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
