use animethemes_server_rust::{entities::content::animethemeentry, scopes::without_trashed};
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::graphql::{
    inputs::pagination_input::PaginationInput, types::content::animethemeentry::AnimeThemeEntry,
    utils::cursor_paginate,
};

#[derive(InputObject, Default)]
pub struct AnimeThemeEntryFilterInput {
    spoiler: Option<bool>,
}

#[derive(Default)]
pub struct AnimeThemeEntryQuery;

#[Object]
impl AnimeThemeEntryQuery {
    pub async fn most_popular_entries(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<AnimeThemeEntryFilterInput>,
    ) -> Result<Connection<u64, AnimeThemeEntry, EmptyFields, EmptyFields>> {
        let mut query =
            animethemeentry::Entity::find().filter(without_trashed::<animethemeentry::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(spoiler) = filter.spoiler {
            query = query.filter(animethemeentry::Column::Spoiler.eq(spoiler));
        }

        query = query.order_by_desc(animethemeentry::Column::TracksCount);

        cursor_paginate(
            query,
            ctx,
            animethemeentry::Column::Id,
            pagination,
            |model: &animethemeentry::Model| model.id,
        )
        .await
    }
}
