use animethemes_server_rust::entities::auth::role;
use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::graphql::{
    loaders::auth::role::role_permissions::RolePermissionsLoader,
    types::auth::permission::Permission,
};

/// Represents an assignable label for users that provides a configured group of permissions.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Role {
    /// The primary key of the resource
    pub id: u64,
    /// The label of the resource
    pub name: String,
    /// The hex representation of the color used to distinguish the resource
    pub color: Option<String>,
    /// Is the role assigned on account verification?
    pub default: bool,
    /// The weight assigned to the resource, where higher values correspond to higher priority
    pub priority: i32,
}

impl From<role::Model> for Role {
    fn from(model: role::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            color: model.color,
            default: model.default,
            priority: model.priority,
        }
    }
}

#[ComplexObject]
impl Role {
    async fn permissions(&self, ctx: &Context<'_>) -> Result<Vec<Permission>> {
        let loader = ctx.data::<DataLoader<RolePermissionsLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Permission::from).collect())
    }
}
