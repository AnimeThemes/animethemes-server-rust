use crate::{
    AppError,
    enums::{LocalizedEnum, list::playlistvisibility::PlaylistVisibility},
    middlewares::current_user::CurrentUser,
    policies::{
        Policy, PolicyAction,
        list::{playlist::PlaylistPolicy, track::PlaylistTrackPolicy},
    },
};
use async_graphql::{
    ComplexObject, Context, Error, InputObject, Object, Result, SimpleObject,
    dataloader::DataLoader,
};

use crate::{
    entities::list::playlist::{self},
    graphql::{
        enums::sort::list::track_sort::PlaylistTrackSort,
        loaders::list::playlist::{
            playlist_tracks::{PlaylistTracksLoader, PlaylistTracksLoaderKey},
            playlist_tracks_count::PlaylistTracksCountLoader,
            playlist_user::PlaylistUserLoader,
        },
        types::{auth::user::User, list::track::PlaylistTrack},
    },
};

#[derive(InputObject, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlaylistTracksFilterInput {
    pub entry_id: Option<u64>,
    pub video_id: Option<u64>,
}

/// Represents a list of ordered tracks intended for continuous playback.
///
/// For example, a "/r/anime's Best OPs and EDs of 2022" playlist may contain a collection of tracks allowing the continuous playback of Best OP and ED nominations for the /r/anime Awards.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Playlist {
    /// The primary key of the resource
    #[graphql(name = "id")]
    pub hashid: String,
    #[graphql(skip)]
    pub id: u64,
    #[graphql(skip)]
    pub user_id: u64,
    /// The title of the playlist
    pub name: String,
    /// The description of the playlist
    pub description: Option<String>,
    /// The state of who can see the playlist
    pub visibility: PlaylistVisibility,
    /// The localized string value of the visibility field
    pub visibility_localized: String,
    #[graphql(skip)]
    pub model: playlist::Model,
}

#[ComplexObject]
impl Playlist {
    async fn tracks_count(&self, ctx: &Context<'_>) -> Result<i32> {
        let loader = ctx.data::<DataLoader<PlaylistTracksCountLoader>>()?;

        let count = loader.load_one(self.id).await?.unwrap_or(0);

        Ok(count as i32)
    }

    async fn tracks_exists(&self, ctx: &Context<'_>) -> Result<bool> {
        let loader = ctx.data::<DataLoader<PlaylistTracksCountLoader>>()?;

        let count = loader.load_one(self.id).await?.unwrap_or(0);

        Ok(count > 0)
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let loader = ctx.data::<DataLoader<PlaylistUserLoader>>()?;

        Ok(loader
            .load_one(self.user_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?
            .into())
    }

    async fn tracks(
        &self,
        ctx: &Context<'_>,
        filter: Option<PlaylistTracksFilterInput>,
        sort: Option<Vec<PlaylistTrackSort>>,
    ) -> Result<Vec<PlaylistTrack>> {
        let loader = ctx.data::<DataLoader<PlaylistTracksLoader>>()?;

        let models = loader
            .load_one(PlaylistTracksLoaderKey::new(self.id, filter, sort))
            .await?
            .unwrap_or_default();

        Ok(models.into_iter().map(PlaylistTrack::from).collect())
    }

    async fn permissions(&self, _ctx: &Context<'_>) -> PlaylistPermissions {
        PlaylistPermissions {
            playlist: self.model.clone(),
        }
    }
}

impl From<playlist::Model> for Playlist {
    fn from(model: playlist::Model) -> Self {
        Self {
            model: model.clone(),
            hashid: model.hashid.unwrap(),
            id: model.id,
            user_id: model.user_id,
            name: model.name,
            description: model.description,
            visibility: model.visibility,
            visibility_localized: model.visibility.localize().to_string(),
        }
    }
}

struct PlaylistPermissions {
    playlist: playlist::Model,
}

#[Object]
impl PlaylistPermissions {
    async fn can_update(&self, ctx: &Context<'_>) -> bool {
        let Some(user) = ctx.data_opt::<CurrentUser>() else {
            return false;
        };

        match PlaylistPolicy::check(Some(&user), PolicyAction::Update, Some(&self.playlist))
            .authorize()
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn can_delete(&self, ctx: &Context<'_>) -> bool {
        let Some(user) = ctx.data_opt::<CurrentUser>() else {
            return false;
        };

        match PlaylistPolicy::check(Some(&user), PolicyAction::Delete, Some(&self.playlist))
            .authorize()
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn can_reorder_tracks(&self, ctx: &Context<'_>) -> bool {
        let Some(user) = ctx.data_opt::<CurrentUser>() else {
            return false;
        };

        match PlaylistTrackPolicy::check(Some(&user), PolicyAction::Update, Some(&self.playlist))
            .authorize()
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
