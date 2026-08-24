use std::fmt::{self, Display, Formatter};

pub enum Feature {
    Registration,
    AllowPlaylistManagement,
}

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Feature::Registration => "registration",
            Feature::AllowPlaylistManagement => "allow-playlist-management",
        };

        f.write_str(name)
    }
}
