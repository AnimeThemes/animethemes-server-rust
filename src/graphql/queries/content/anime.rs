use animethemes_server_rust::enums::content::{
    animeformat::AnimeFormat as AnimeFormatEnum, animeseason::AnimeSeason as AnimeSeasonEnum,
};
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::content::anime,
    graphql::{
        enums::{
            content::{animeformat::AnimeFormat, animeseason::AnimeSeason},
            sort::{GraphQLSort, content::anime_sort::AnimeSort},
        },
        inputs::pagination_input::PaginationInput,
        types::content::anime::Anime,
        utils::cursor_paginate,
    },
    scopes::without_trashed,
};

#[derive(InputObject, Default)]
pub struct AnimeFilterInput {
    title_like: Option<String>,
    season: Option<AnimeSeason>,
    year: Option<i16>,
    format: Option<AnimeFormat>,
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

        Ok(anime.map(Into::into))
    }

    pub async fn anime_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<AnimeFilterInput>,
        sort: Option<Vec<AnimeSort>>,
        _search: Option<String>,
    ) -> Result<Connection<u64, Anime, EmptyFields, EmptyFields>> {
        let mut query = anime::Entity::find().filter(without_trashed::<anime::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(title_like) = filter.title_like {
            query = query.filter(anime::Column::Title.like(title_like))
        }

        if let Some(season) = filter.season {
            query = query.filter(anime::Column::Season.eq::<AnimeSeasonEnum>(season.into()))
        }

        if let Some(year) = filter.year {
            query = query.filter(anime::Column::Year.eq(year))
        }

        if let Some(format) = filter.format {
            query = query.filter(anime::Column::Format.eq::<AnimeFormatEnum>(format.into()))
        }

        if let Some(animeyear_season) = filter.animeyear_season {
            query =
                query.filter(anime::Column::Season.eq::<AnimeSeasonEnum>(animeyear_season.into()));
        }

        if let Some(animeyear_year) = filter.animeyear_year {
            query = query.filter(anime::Column::Year.eq(animeyear_year))
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
