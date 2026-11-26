use mod_events::Event;

use crate::entities::auth::user;

#[derive(Debug, Clone)]
pub struct UserVerified {
    pub user: user::Model,
}

impl Event for UserVerified {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
