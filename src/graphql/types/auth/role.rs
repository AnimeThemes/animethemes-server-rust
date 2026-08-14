use animethemes_graphql_rust::entities::auth::role;
use async_graphql::SimpleObject;

/// Represents an assignable label for users that provides a configured group of permissions.
#[derive(SimpleObject)]
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
