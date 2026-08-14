use animethemes_server_rust::entities::content::series;
use async_graphql::Enum;
use sea_orm::{QueryOrder, Select, sea_query::Expr};

use crate::graphql::enums::sort::GraphQLSort;

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

impl GraphQLSort<series::Entity> for SeriesSort {
    fn apply_sort(&self, query: Select<series::Entity>) -> Select<series::Entity> {
        match self {
            SeriesSort::Id => query.order_by_asc(series::Column::Id),
            SeriesSort::IdDesc => query.order_by_desc(series::Column::Id),
            SeriesSort::TitleRomaji => query.order_by_asc(series::Column::Title),
            SeriesSort::TitleRomajiDesc => query.order_by_desc(series::Column::Title),
            SeriesSort::CreatedAt => query.order_by_asc(series::Column::CreatedAt),
            SeriesSort::CreatedAtDesc => query.order_by_desc(series::Column::CreatedAt),
            SeriesSort::UpdatedAt => query.order_by_asc(series::Column::UpdatedAt),
            SeriesSort::UpdatedAtDesc => query.order_by_desc(series::Column::UpdatedAt),
            SeriesSort::Random => query.order_by_asc(Expr::cust("RAND()")),
        }
    }
}
