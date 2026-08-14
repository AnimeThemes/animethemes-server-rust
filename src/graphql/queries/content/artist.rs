use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::content::artist,
    graphql::{
        enums::sort::{GraphQLSort, content::artist_sort::ArtistSort},
        inputs::pagination_input::PaginationInput,
        types::content::artist::Artist,
        utils::cursor_paginate,
    },
    scopes::without_trashed,
};

#[derive(InputObject, Default)]
struct ArtistFilterInput {
    name_main_like: Option<String>,
}

#[derive(Default)]
pub struct ArtistQuery;

#[Object]
impl ArtistQuery {
    async fn artist(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Artist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let artist = artist::Entity::find()
            .filter(without_trashed::<artist::Entity>())
            .filter(artist::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(artist.map(Into::into))
    }

    async fn artist_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<ArtistFilterInput>,
        sort: Option<Vec<ArtistSort>>,
    ) -> Result<Connection<u64, Artist, EmptyFields, EmptyFields>> {
        let mut query = artist::Entity::find().filter(without_trashed::<artist::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(name_main_like) = filter.name_main_like {
            query = query.filter(artist::Column::Name.like(name_main_like))
        }

        if let Some(sorts) = sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        cursor_paginate(
            query,
            ctx,
            artist::Column::Id,
            pagination,
            |model: &artist::Model| model.id,
        )
        .await
    }
}
