use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};
use validator::{Validate, ValidationError};

use crate::actions::entities::{nullable, nullable_string, optional, optional_string};
use crate::entities::content::song_staff;
use crate::events::content::songstaff_created::SongStaffCreated;
use crate::events::content::songstaff_deleting::SongStaffDeleting;
use crate::events::content::songstaff_updated::SongStaffUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::*;
use crate::traits::soft_delete::SoftDeleteModel;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "SaveSongStaffInput")]
pub struct SaveSongStaffInput {
    pub song_id: Option<u64>,
    pub artist_id: Option<u64>,
    pub member_id: MaybeUndefined<u64>,

    #[validate(length(
        min = 1,
        max = 192,
        message = "The role must be between 1 and 192 characters."
    ))]
    pub role: Option<String>,

    #[validate(custom(
        function = "string::validate_length::<1, 192>",
        message = "The alias must be between 1 and 192 characters."
    ))]
    pub alias: MaybeUndefined<String>,

    #[validate(custom(
        function = "string::validate_length::<1, 192>",
        message = "The as must be between 1 and 192 characters."
    ))]
    pub r#as: MaybeUndefined<String>,
}

pub struct SaveSongStaffAction;

impl SaveSongStaffAction {
    pub async fn save(
        ctx: &AppContext,
        staff: Option<song_staff::Model>,
        input: SaveSongStaffInput,
        user: &CurrentUser,
    ) -> Result<song_staff::Model, AppError> {
        let mut errors = input.validate().err().unwrap_or_default();

        if staff.is_none() {
            if input.song_id.is_none() {
                errors.add(
                    "songId",
                    ValidationError::new("required").with_message("The songId is required.".into()),
                );
            }

            if input.artist_id.is_none() {
                errors.add(
                    "artistId",
                    ValidationError::new("required")
                        .with_message("The artistId is required.".into()),
                );
            }
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let old_staff = staff.as_ref().cloned();

        let staff = song_staff::ActiveModel {
            id: optional(staff.as_ref().map(|s| s.id)),
            song_id: optional(input.song_id),
            artist_id: optional(input.artist_id),
            member_id: nullable(input.member_id),
            role: optional_string(input.role),
            alias: nullable_string(input.alias),
            r#as: nullable_string(input.r#as),
            ..Default::default()
        };

        let staff = staff.save(&ctx.db).await?.try_into_model()?;

        if let Some(old_staff) = old_staff {
            events::dispatch(
                ctx,
                SongStaffUpdated {
                    old_staff: old_staff,
                    new_staff: staff.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        } else {
            events::dispatch(
                ctx,
                SongStaffCreated {
                    staff: staff.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        }

        Ok(staff)
    }

    pub async fn delete(
        ctx: &AppContext,
        staff: song_staff::Model,
        is_force_delete: bool,
        user: &CurrentUser,
    ) -> Result<(), AppError> {
        events::dispatch(
            ctx,
            SongStaffDeleting {
                staff: staff.clone(),
                is_force_deleting: is_force_delete,
                user: Some(user.clone()),
            },
        )
        .await;

        if is_force_delete {
            staff.delete(&ctx.db).await?;
        } else {
            staff.soft_delete(&ctx.db).await?;
        }

        Ok(())
    }
}
