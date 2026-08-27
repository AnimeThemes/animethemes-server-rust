use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::{
    AppError,
    entities::auth::{
        model_has_roles,
        role::{self, Roles},
        user,
    },
};

pub struct RoleAction;

impl RoleAction {
    async fn find_role(db: &DatabaseConnection, role: Roles) -> Result<role::Model, AppError> {
        role::Entity::find_by_name(role.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound)
    }

    pub async fn assign_role(
        db: &DatabaseConnection,
        user: &user::Model,
        role: Roles,
    ) -> Result<(), AppError> {
        let role_id = Self::find_role(db, role).await?.id;

        let exists = model_has_roles::Entity::find()
            .filter(model_has_roles::Column::ModelType.eq("user"))
            .filter(model_has_roles::Column::ModelId.eq(user.id))
            .filter(model_has_roles::Column::RoleId.eq(role_id))
            .one(db)
            .await?
            .is_some();

        if exists {
            return Ok(());
        }

        model_has_roles::ActiveModel {
            model_type: Set("user".to_string()),
            model_id: Set(user.id),
            role_id: Set(role_id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    pub async fn remove_role(
        db: &DatabaseConnection,
        user: &user::Model,
        role: Roles,
    ) -> Result<(), AppError> {
        let role_id = Self::find_role(db, role).await?.id;

        model_has_roles::Entity::delete_many()
            .filter(model_has_roles::Column::ModelType.eq("user"))
            .filter(model_has_roles::Column::ModelId.eq(user.id))
            .filter(model_has_roles::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;

        Ok(())
    }
}
