use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ActiveValue, ModelTrait, TryIntoModel};
use validator::Validate;

use crate::actions::entities::{nullable_string, optional, optional_string};
use crate::entities::content::song;
use crate::events::content::song_created::SongCreated;
use crate::events::content::song_deleting::SongDeleting;
use crate::events::content::song_updated::SongUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::{validate_required_string, *};
use crate::traits::soft_delete::SoftDeleteModel;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "SaveSongInput")]
pub struct SaveSongInput {
    #[validate(nested)]
    pub title: Option<SongTitleInput>,
}

#[derive(Default, InputObject, Validate)]
#[graphql(name = "SongTitleInput")]
pub struct SongTitleInput {
    #[validate(length(
        min = 1,
        max = 192,
        message = "The romaji title must be between 1 and 192 characters."
    ))]
    pub romaji: Option<String>,

    #[validate(custom(
        function = "string::validate_length::<1, 192>",
        message = "The native title must be between 1 and 192 characters."
    ))]
    pub native: MaybeUndefined<String>,
}

pub struct SaveSongAction;

impl SaveSongAction {
    pub async fn save(
        ctx: &AppContext,
        song: Option<song::Model>,
        input: SaveSongInput,
        user: &CurrentUser,
    ) -> Result<song::Model, AppError> {
        let mut errors = input.validate().err().unwrap_or_default();

        if song.is_none() {
            validate_required_string(
                &mut errors,
                "title.romaji",
                input.title.as_ref().and_then(|t| t.romaji.as_deref()),
                "The romaji title is required.",
            );
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let title = input.title.unwrap_or_default();

        let old_song = song.as_ref().cloned();

        let song = song::ActiveModel {
            id: optional(song.as_ref().map(|s| s.id)),
            title: match optional_string(title.romaji) {
                ActiveValue::Set(value) => ActiveValue::Set(Some(value)),
                ActiveValue::Unchanged(value) => ActiveValue::Unchanged(Some(value)),
                ActiveValue::NotSet => ActiveValue::NotSet,
            },
            title_native: nullable_string(title.native),
            ..Default::default()
        };

        let song = song.save(&ctx.db).await?.try_into_model()?;

        if let Some(old_song) = old_song {
            events::dispatch(
                ctx,
                SongUpdated {
                    old_song: old_song,
                    new_song: song.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        } else {
            events::dispatch(
                ctx,
                SongCreated {
                    song: song.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        }

        Ok(song)
    }

    pub async fn delete(
        ctx: &AppContext,
        song: song::Model,
        is_force_delete: bool,
        user: &CurrentUser,
    ) -> Result<(), AppError> {
        events::dispatch(
            ctx,
            SongDeleting {
                song: song.clone(),
                is_force_deleting: is_force_delete,
                user: Some(user.clone()),
            },
        )
        .await;

        if is_force_delete {
            song.delete(&ctx.db).await?;
        } else {
            song.soft_delete(&ctx.db).await?;
        }

        Ok(())
    }
}
