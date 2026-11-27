use mod_events::Event;

use crate::entities::content::song_staff;
use crate::middlewares::current_user::CurrentUser;

#[derive(Debug, Clone)]
pub struct SongStaffDeleting {
    pub staff: song_staff::Model,
    pub is_force_deleting: bool,
    pub user: Option<CurrentUser>,
}

impl Event for SongStaffDeleting {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
