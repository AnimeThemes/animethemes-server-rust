use async_graphql::SimpleObject;

use crate::entities::content::themegroup;

/// Represents the group that accompanies a Theme.
///For example, English Version is the group for english dubbed Theme.For example, Staple Stable is the song for the Bakemonogatari OP1 Theme.
#[derive(SimpleObject)]
pub struct ThemeGroup {
    /// The primary key of the resource
    pub id: u64,
    /// The name of the group
    pub name: String,
    /// The slug of the group
    pub slug: String,
}

impl From<themegroup::Model> for ThemeGroup {
    fn from(model: themegroup::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
        }
    }
}
