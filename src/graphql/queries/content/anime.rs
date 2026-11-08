use crate::{
    AppError,
    entities::content::{externalresource, resourceable},
    enums::content::{
        animeformat::AnimeFormat, animeseason::AnimeSeason, resourcesite::ResourceSite,
    },
    rules::validation_error::ValidationError,
};
use async_graphql::{
    Context, InputObject, Object, Result, ResultExt,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, Order, QueryFilter, QuerySelect,
};

use crate::{
    entities::content::anime,
    graphql::{
        cursor::{CursorSort, PaginationCursor, cursor_paginate},
        enums::sort::{GraphQLSort, content::anime_sort::AnimeSort},
        inputs::pagination_input::PaginationInput,
        types::content::anime::Anime,
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
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Anime, EmptyFields, EmptyFields>> {
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let mut query = anime::Entity::find().filter(without_trashed::<anime::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(title_like) = filter.title_like {
            query = query.filter(anime::Column::Title.like(title_like))
        }

        if let Some(season) = filter.season {
            query = query.filter(anime::Column::Season.eq(season))
        }

        if let Some(year) = filter.year {
            query = query.filter(anime::Column::Year.eq(year))
        }

        if let Some(format) = filter.format {
            query = query.filter(anime::Column::Format.eq(format))
        }

        if let Some(animeyear_season) = filter.animeyear_season {
            query = query.filter(anime::Column::Season.eq(animeyear_season));
        }

        if let Some(animeyear_year) = filter.animeyear_year {
            query = query.filter(anime::Column::Year.eq(animeyear_year))
        }

        if let Some(sorts) = sort.clone() {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(AnimeSort::cursor_sort)
            .collect::<Vec<_>>();

        cursor_sorts.push(CursorSort {
            column: anime::Column::Id,
            order: Order::Asc,
        });

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }

    /// Filter anime by its external id on given site
    pub async fn filter_anime_by_external_site(
        &self,
        ctx: &Context<'_>,
        site: ResourceSite,
        id: Option<Vec<u64>>,
        link: Option<String>,
    ) -> Result<Vec<Anime>> {
        let mut errors = Vec::new();

        let mut id_errors = Vec::new();

        let has_id = id.as_ref().is_some_and(|ids| !ids.is_empty());
        let has_link = link.as_ref().is_some_and(|link| !link.trim().is_empty());

        if !has_id && !has_link {
            id_errors.push("Either 'id' or 'link' must be provided");

            errors.push(ValidationError::new(
                "link",
                vec!["Either 'id' or 'link' must be provided"],
            ));
        }

        if id.as_ref().is_some_and(|ids| ids.len() > 100) {
            id_errors.push("'Id' must contain at most 100 entries");
        }

        if !id_errors.is_empty() {
            errors.push(ValidationError::new("id", id_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors)).extend()?;
        }

        let mut query = anime::Entity::find()
            .filter(without_trashed::<anime::Entity>())
            .join_rev(
                JoinType::InnerJoin,
                resourceable::Entity::belongs_to(anime::Entity)
                    .from(resourceable::Column::ResourceableId)
                    .to(anime::Column::Id)
                    .into(),
            )
            .join(
                JoinType::InnerJoin,
                resourceable::Entity::belongs_to(externalresource::Entity)
                    .from(resourceable::Column::ResourceId)
                    .to(externalresource::Column::Id)
                    .into(),
            )
            .filter(resourceable::Column::ResourceableType.eq("anime"))
            .filter(externalresource::Column::Site.eq(site))
            .distinct();

        if let Some(ids) = id {
            query = query.filter(externalresource::Column::ExternalId.is_in(ids));
        }

        if let Some(link) = link {
            query = query.filter(externalresource::Column::Link.eq(link));
        }

        let db = ctx.data::<DatabaseConnection>()?;
        let models = query.all(db).await?;

        Ok(models.into_iter().map(Anime::from).collect())
    }
}
