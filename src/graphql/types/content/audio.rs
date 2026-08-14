use async_graphql::SimpleObject;

use crate::entities::content::audio;

/// Represents the audio track of a video.
///
/// For example, the audio Bakemonogatari-OP1.ogg represents the audio track of the Bakemonogatari-OP1.webm video.
#[derive(SimpleObject)]
pub struct Audio {
    /// The primary key of the resource
    pub id: u64,
    /// The basename of the file in storage
    pub basename: String,
    /// The filename of the file in storage
    pub filename: String,
    /// The URL to stream the file from storage
    pub link: String,
    /// The media type of the file in storage
    pub mimetype: String,
    /// The path of the file in storage
    pub path: String,
    /// The size of the file in storage in Bytes
    pub size: i32,
}

impl From<audio::Model> for Audio {
    fn from(model: audio::Model) -> Self {
        Self {
            id: model.id,
            basename: model.basename.clone(),
            filename: model.filename.clone(),
            link: model.link(),
            mimetype: model.mimetype,
            path: model.path,
            size: model.size,
        }
    }
}
