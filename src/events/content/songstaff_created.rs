use mod_events::Event;

use crate::entities::content::song_staff;
use crate::middlewares::current_user::CurrentUser;

#[derive(Debug, Clone)]
pub struct SongStaffCreated {
    pub staff: song_staff::Model,
    pub user: Option<CurrentUser>,
}

impl Event for SongStaffCreated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
