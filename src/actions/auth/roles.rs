use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    AppError,
    entities::auth::{
        role::{self, Roles},
        user, user_roles,
    },
};

pub struct RoleAction;

impl RoleAction {
    async fn find_role(ctx: &AppContext, role: Roles) -> Result<role::Model, AppError> {
        role::Entity::find_by_name(role.to_string())
            .one(&ctx.db)
            .await?
            .ok_or_else(|| AppError::NotFound)
    }

    pub async fn assign_role(
        ctx: &AppContext,
        user: &user::Model,
        role: Roles,
    ) -> Result<(), AppError> {
        let role_id = Self::find_role(ctx, role).await?.id;

        let exists = user_roles::Entity::find()
            .filter(user_roles::Column::UserId.eq(user.id))
            .filter(user_roles::Column::RoleId.eq(role_id))
            .one(&ctx.db)
            .await?
            .is_some();

        if exists {
            return Ok(());
        }

        user_roles::ActiveModel {
            user_id: Set(user.id),
            role_id: Set(role_id),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await?;

        Ok(())
    }

    pub async fn remove_role(
        ctx: &AppContext,
        user: &user::Model,
        role: Roles,
    ) -> Result<(), AppError> {
        let role_id = Self::find_role(ctx, role).await?.id;

        user_roles::Entity::delete_many()
            .filter(user_roles::Column::UserId.eq(user.id))
            .filter(user_roles::Column::RoleId.eq(role_id))
            .exec(&ctx.db)
            .await?;

        Ok(())
    }
}
