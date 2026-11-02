use std::fmt::{self, Display, Formatter};

pub enum Feature {
    Registration,
    AllowPlaylistManagement,
    AllowAudioStreams,
    AllowVideoStreams,
}

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Feature::Registration => "registration",
            Feature::AllowPlaylistManagement => "allow-playlist-management",
            Feature::AllowAudioStreams => "allow-audio-streams",
            Feature::AllowVideoStreams => "allow-video-streams",
        };

        f.write_str(name)
    }
}
