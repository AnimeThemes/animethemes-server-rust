use animethemes_graphql_rust::entities::auth::permission;
use async_graphql::SimpleObject;

/// Represents an assignable label for users and roles that authorizes a particular action in AnimeThemes.
#[derive(SimpleObject)]
pub struct Permission {
    /// The primary key of the resource
    pub id: u64,
    /// The label of the resource
    pub name: String,
}

impl From<permission::Model> for Permission {
    fn from(model: permission::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}
