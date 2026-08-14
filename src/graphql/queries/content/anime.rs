use animethemes_server_rust::enums::content::animeseason::AnimeSeason as AnimeSeasonEnum;
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::content::anime,
    graphql::{
        enums::{
            content::animeseason::AnimeSeason,
            sort::{GraphQLSort, content::anime_sort::AnimeSort},
        },
        inputs::pagination_input::PaginationInput,
        types::content::anime::Anime,
        utils::cursor_paginate,
    },
    scopes::{
        content::anime::{anime_by_title_like, anime_by_year},
        without_trashed,
    },
};

#[derive(InputObject, Default)]
pub struct AnimeFilterInput {
    title_like: Option<String>,
    #[graphql(skip)]
    pub animeyear_season: Option<AnimeSeason>,
    #[graphql(skip)]
    pub animeyear_year: Option<i16>,
}

#[derive(Default)]
pub struct AnimeQuery;

#[Object]
impl AnimeQuery {
    async fn anime(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let anime = anime::Entity::find()
            .filter(without_trashed::<anime::Entity>())
            .filter(anime::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(anime.map(|a| a.into()))
    }

    pub async fn anime_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<AnimeFilterInput>,
        sort: Option<Vec<AnimeSort>>,
    ) -> Result<Connection<u64, Anime, EmptyFields, EmptyFields>> {
        let mut query = anime::Entity::find().filter(without_trashed::<anime::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(title_like) = filter.title_like {
            query = query.filter(anime_by_title_like(title_like))
        }

        if let Some(animeyear_season) = filter.animeyear_season {
            query = query.filter(anime::Column::Season.eq(AnimeSeasonEnum::from(animeyear_season)));
        }

        if let Some(animeyear_year) = filter.animeyear_year {
            query = query.filter(anime_by_year(animeyear_year))
        }

        if let Some(sorts) = sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        cursor_paginate(
            query,
            ctx,
            anime::Column::Id,
            pagination,
            |model: &anime::Model| model.id,
        )
        .await
    }
}
