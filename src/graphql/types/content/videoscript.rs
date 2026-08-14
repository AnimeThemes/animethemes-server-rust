use async_graphql::SimpleObject;

use crate::entities::content::videoscript;

/// Represents an encoding script used to produce a video.
///
/// For example, the 2009/Summer/Bakemonogatari-OP1.txt video script represents the encoding script of the Bakemonogatari-OP1.webm video.
#[derive(SimpleObject)]
pub struct VideoScript {
    /// The primary key of the resource
    pub id: u64,
    /// The path of the file in storage
    pub path: String,
    /// The URL to stream the file from storage
    pub link: String,
}

impl From<videoscript::Model> for VideoScript {
    fn from(model: videoscript::Model) -> Self {
        Self {
            id: model.id,
            path: model.path.clone(),
            link: model.link(),
        }
    }
}
