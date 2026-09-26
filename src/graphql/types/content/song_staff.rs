use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::content::song_staff,
    graphql::{
        loaders::content::songstaff::{
            songstaff_artist::SongStaffArtistLoader, songstaff_member::SongStaffMemberLoader,
            songstaff_song::SongStaffSongLoader,
        },
        types::content::{artist::Artist, song::Song},
    },
};

/// Represents the link between a song and an artist or group.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct SongStaff {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub song_id: u64,
    #[graphql(skip)]
    pub artist_id: u64,
    #[graphql(skip)]
    pub member_id: Option<u64>,
    /// The alias the artist is using for this staff
    pub alias: Option<String>,
    /// The character the artist is performing as
    pub r#as: Option<String>,
    /// The alias the member is using for this staff
    pub member_alias: Option<String>,
    /// The character the member is performing as
    pub member_as: Option<String>,
    /// Used to determine the relevance order of artists in staffs
    pub relevance: i32,
    /// The role the artist is performing
    pub role: String,
}

#[ComplexObject]
impl SongStaff {
    async fn artist(&self, ctx: &Context<'_>) -> Result<Artist> {
        let loader = ctx.data_unchecked::<DataLoader<SongStaffArtistLoader>>();

        let artist = loader
            .load_one(self.artist_id)
            .await?
            .ok_or("Artist not found")?;

        Ok(artist.into())
    }

    async fn member(&self, ctx: &Context<'_>) -> Result<Option<Artist>> {
        let Some(member_id) = self.member_id else {
            return Ok(None);
        };

        let loader = ctx.data_unchecked::<DataLoader<SongStaffMemberLoader>>();

        Ok(loader.load_one(member_id).await?.map(Into::into))
    }

    async fn song(&self, ctx: &Context<'_>) -> Result<Song> {
        let loader = ctx.data_unchecked::<DataLoader<SongStaffSongLoader>>();

        let song = loader
            .load_one(self.song_id)
            .await?
            .ok_or("Song not found")?;

        Ok(song.into())
    }
}

impl From<song_staff::Model> for SongStaff {
    fn from(model: song_staff::Model) -> Self {
        Self {
            id: model.id,
            song_id: model.song_id,
            artist_id: model.artist_id,
            member_id: model.member_id,
            alias: model.alias,
            r#as: model.r#as,
            member_alias: model.member_alias,
            member_as: model.member_as,
            relevance: model.relevance,
            role: model.role,
        }
    }
}
