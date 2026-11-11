use std::collections::BTreeMap;

use async_graphql::connection::{Connection, EmptyFields, OpaqueCursor};
use async_graphql::{ComplexObject, Context, Object, Result, ResultExt, SimpleObject};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::entities::content::anime;
use crate::enums::LocalizedEnum;
use crate::enums::content::animeseason::AnimeSeason;
use crate::graphql::cursor::PaginationCursor;
use crate::graphql::enums::sort::content::anime_sort::AnimeSort;
use crate::graphql::inputs::pagination_input::PaginationInput;
use crate::graphql::queries::content::anime::{AnimeFilterInput, AnimeQuery};
use crate::graphql::types::content::anime::Anime;

/// The anime year response type, grouped by season
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AnimeYear {
    /// The year of the AnimeYear type
    year: i16,
    #[graphql(skip)]
    available_seasons: Vec<AnimeSeason>,
}

#[ComplexObject]
impl AnimeYear {
    /// The available seasons of the year and its anime
    async fn season(&self, season: Option<AnimeSeason>) -> Vec<AnimeYearSeason> {
        let mut seasons = self.available_seasons.clone();

        if let Some(season) = season {
            seasons.retain(|s| *s == season);
        }

        seasons
            .iter()
            .copied()
            .map(|season| AnimeYearSeason {
                season,
                season_localized: season.localize().to_string(),
                year: self.year,
            })
            .collect()
    }
}

/// The anime year season type.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AnimeYearSeason {
    /// The season of the anime year.
    season: AnimeSeason,
    /// The formatted string value of the season field.
    season_localized: String,
    #[graphql(skip)]
    year: i16,
}

#[ComplexObject]
impl AnimeYearSeason {
    async fn anime(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<AnimeFilterInput>,
        sort: Option<Vec<AnimeSort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Anime, EmptyFields, EmptyFields>> {
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let mut filter = filter.unwrap_or_default();

        filter.animeyear_year = Some(self.year);
        filter.animeyear_season = Some(self.season.clone().into());

        AnimeQuery::default()
            .anime_connection(ctx, pagination, Some(filter), sort)
            .await
    }
}

#[derive(Default)]
pub struct AnimeYearQuery;

#[Object]
impl AnimeYearQuery {
    /// Returns a list of years grouped by its seasons.
    async fn animeyears(
        &self,
        ctx: &Context<'_>,
        year: Option<Vec<i16>>,
    ) -> Result<Vec<AnimeYear>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let mut query = anime::Entity::find()
            .select_only()
            .column(anime::Column::Year)
            .column(anime::Column::Season)
            .filter(anime::Column::Year.is_not_null())
            .filter(anime::Column::Season.is_not_null())
            .distinct()
            .order_by_asc(anime::Column::Year)
            .order_by_asc(anime::Column::Season);

        if let Some(years) = year.filter(|years| !years.is_empty()) {
            query = query.filter(anime::Column::Year.is_in(years));
        }

        let rows = query
            .into_tuple::<(Option<i16>, Option<AnimeSeason>)>()
            .all(db)
            .await?;

        let mut grouped: BTreeMap<i16, Vec<AnimeSeason>> = BTreeMap::new();

        for (year, season) in rows {
            let (Some(year), Some(season)) = (year, season) else {
                continue;
            };

            let season: AnimeSeason = season.into();

            let seasons = grouped.entry(year).or_default();

            if !seasons.contains(&season) {
                seasons.push(season);
            }
        }

        Ok(grouped
            .into_iter()
            .map(|(year, available_seasons)| AnimeYear {
                year,
                available_seasons,
            })
            .collect())
    }
}
