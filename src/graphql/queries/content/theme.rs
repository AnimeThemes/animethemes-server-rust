use crate::{
    entities::content::{anime, entry, theme},
    enums::content::{animeformat::AnimeFormat, themetype::ThemeType},
    scopes::without_trashed,
};
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, Order, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait, sea_query::Expr,
};

use crate::graphql::{
    cursor::{CursorSort, PaginationCursor, cursor_paginate},
    enums::sort::{GraphQLSort, content::theme_sort::ThemeSort},
    inputs::pagination_input::PaginationInput,
    types::content::theme::Theme,
};

#[derive(InputObject, Default)]
struct ThemeShuffleInput {
    r#type: Option<Vec<ThemeType>>,
    format: Option<AnimeFormat>,
    year_gte: Option<i16>,
    year_lte: Option<i16>,
    spoiler: Option<bool>,
}

#[derive(InputObject, Default)]
pub struct ThemeFilterInput {
    id_in: Option<Vec<u64>>,
    r#type: Option<ThemeType>,
}

#[derive(Default)]
pub struct ThemeQuery;

#[Object]
impl ThemeQuery {
    async fn theme_shuffle(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 15)] first: u64,
        input: Option<ThemeShuffleInput>,
    ) -> Result<Vec<Theme>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let input = input.unwrap_or_default();

        let mut query = theme::Entity::find()
            .join(JoinType::InnerJoin, theme::Relation::Anime.def())
            .join(JoinType::InnerJoin, theme::Relation::Entry.def());

        if let Some(r#type) = input.r#type {
            query = query.filter(theme::Column::Type.is_in(r#type));
        }

        if let Some(format) = input.format {
            query = query.filter(anime::Column::Format.eq(format));
        }

        if let Some(year_gte) = input.year_gte {
            query = query.filter(anime::Column::Year.gte(year_gte));
        }

        if let Some(year_lte) = input.year_lte {
            query = query.filter(anime::Column::Year.lte(year_lte));
        }

        if let Some(spoiler) = input.spoiler {
            query = query.filter(entry::Column::Spoiler.eq(spoiler));
        }

        let themes = query
            .distinct()
            .order_by_asc(Expr::cust("RAND()"))
            .limit(first)
            .all(db)
            .await?;

        Ok(themes.into_iter().map(Into::into).collect())
    }

    pub async fn theme_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<ThemeFilterInput>,
        sort: Option<Vec<ThemeSort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Theme, EmptyFields, EmptyFields>> {
        let mut query: sea_orm::prelude::Select<theme::Entity> =
            theme::Entity::find().filter(without_trashed::<theme::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(id_in) = filter.id_in {
            query = query.filter(theme::Column::Id.is_in(id_in))
        }

        if let Some(r#type) = filter.r#type {
            query = query.filter(theme::Column::Type.eq(r#type))
        }

        if let Some(sorts) = sort.clone() {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        let mut cursor_sorts = sort
            .clone()
            .unwrap_or(vec![])
            .iter()
            .filter_map(ThemeSort::cursor_sort)
            .collect::<Vec<_>>();

        cursor_sorts.push(CursorSort {
            column: theme::Column::Id,
            order: Order::Asc,
        });

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
