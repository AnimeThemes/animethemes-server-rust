use mod_events::Event;

use crate::entities::list::track;
use crate::middlewares::current_user::CurrentUser;

#[derive(Debug, Clone)]
pub struct TrackCreated {
    pub track: track::Model,
    pub user: Option<CurrentUser>,
}

impl Event for TrackCreated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
