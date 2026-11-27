use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ActiveValue, ModelTrait, TryIntoModel};
use validator::Validate;

use crate::actions::entities::{optional, optional_string};
use crate::entities::content::artist;
use crate::events::content::artist_created::ArtistCreated;
use crate::events::content::artist_deleting::ArtistDeleting;
use crate::events::content::artist_updated::ArtistUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::validate_required_string;
use crate::traits::soft_delete::SoftDeleteModel;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "SaveArtistInput")]
pub struct SaveArtistInput {
    #[validate(nested)]
    pub name: Option<ArtistNameInput>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The slug must be between 1 and 255 characters."
    ))]
    pub slug: Option<String>,
}

#[derive(Default, InputObject, Validate)]
#[graphql(name = "ArtistNameInput")]
pub struct ArtistNameInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "The main name must be between 1 and 255 characters."
    ))]
    pub main: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The native name must be between 1 and 255 characters."
    ))]
    pub native: Option<String>,
}

pub struct SaveArtistAction;

impl SaveArtistAction {
    pub async fn save(
        ctx: &AppContext,
        artist: Option<artist::Model>,
        input: SaveArtistInput,
        user: &CurrentUser,
    ) -> Result<artist::Model, AppError> {
        let mut errors = input.validate().err().unwrap_or_default();

        if artist.is_none() {
            let name = input.name.as_ref();

            validate_required_string(
                &mut errors,
                "name.main",
                name.and_then(|n| n.main.as_deref()),
                "The name is required.",
            );

            validate_required_string(
                &mut errors,
                "name.native",
                name.and_then(|n| n.native.as_deref()),
                "The native name is required.",
            );

            validate_required_string(
                &mut errors,
                "slug",
                input.slug.as_deref(),
                "The slug is required.",
            );
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let old_artist = artist.as_ref().cloned();

        let name = input.name.unwrap_or_default();

        let artist = artist::ActiveModel {
            id: optional(artist.as_ref().map(|a| a.id)),
            name: optional_string(name.main),
            name_native: match optional_string(name.native) {
                ActiveValue::Set(value) => ActiveValue::Set(Some(value)),
                ActiveValue::Unchanged(value) => ActiveValue::Unchanged(Some(value)),
                ActiveValue::NotSet => ActiveValue::NotSet,
            },
            slug: optional_string(input.slug),
            ..Default::default()
        };

        let artist = artist.save(&ctx.db).await?.try_into_model()?;

        if let Some(old_artist) = old_artist {
            events::dispatch(
                ctx,
                ArtistUpdated {
                    old_artist: old_artist,
                    new_artist: artist.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        } else {
            events::dispatch(
                ctx,
                ArtistCreated {
                    artist: artist.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        }

        Ok(artist)
    }

    pub async fn delete(
        ctx: &AppContext,
        artist: artist::Model,
        is_force_delete: bool,
        user: &CurrentUser,
    ) -> Result<(), AppError> {
        events::dispatch(
            ctx,
            ArtistDeleting {
                artist: artist.clone(),
                is_force_deleting: is_force_delete,
                user: Some(user.clone()),
            },
        )
        .await;

        if is_force_delete {
            artist.delete(&ctx.db).await?;
        } else {
            artist.soft_delete(&ctx.db).await?;
        }

        Ok(())
    }
}
