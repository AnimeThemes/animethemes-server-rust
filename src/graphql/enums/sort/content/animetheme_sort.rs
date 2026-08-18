use animethemes_server_rust::entities::content::animetheme;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

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

impl GraphQLSort<animetheme::Entity> for AnimeThemeSort {
    fn apply_sort(&self, query: Select<animetheme::Entity>) -> Select<animetheme::Entity> {
        match self {
            AnimeThemeSort::Id => query.order_by_asc(animetheme::Column::Id),
            AnimeThemeSort::IdDesc => query.order_by_desc(animetheme::Column::Id),
            AnimeThemeSort::Sequence => query.order_by_asc(animetheme::Column::Sequence),
            AnimeThemeSort::SequenceDesc => query.order_by_desc(animetheme::Column::Sequence),
            AnimeThemeSort::CreatedAt => query.order_by_asc(animetheme::Column::CreatedAt),
            AnimeThemeSort::CreatedAtDesc => query.order_by_desc(animetheme::Column::CreatedAt),
            AnimeThemeSort::UpdatedAt => query.order_by_asc(animetheme::Column::UpdatedAt),
            AnimeThemeSort::UpdatedAtDesc => query.order_by_desc(animetheme::Column::UpdatedAt),
            AnimeThemeSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
