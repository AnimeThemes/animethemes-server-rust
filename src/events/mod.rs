use std::sync::Arc;

use loco_rs::app::AppContext;
use mod_events::{Event, EventDispatcher};

pub mod auth;
pub mod discord;
pub mod list;

#[derive(Clone)]
pub struct EventContext<E> {
    pub ctx: AppContext,
    pub event: E,
}

impl<E: Event> std::fmt::Debug for EventContext<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventContext")
            .field("event", &self.event)
            .finish_non_exhaustive()
    }
}

impl<E: Event> Event for EventContext<E> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub async fn dispatch<E: Event>(ctx: &AppContext, event: E) {
    let dispatcher = ctx
        .shared_store
        .get::<Arc<EventDispatcher>>()
        .expect("EventDispatcher not registered");

    let result = dispatcher
        .dispatch_async_all(EventContext {
            ctx: ctx.clone(),
            event,
        })
        .await;

    for error in result.errors() {
        tracing::error!(
            event = std::any::type_name::<E>(),
            error = %error,
            "Event listener failed"
        );
    }

    if result.is_blocked() {
        tracing::warn!(
            event = std::any::type_name::<E>(),
            "Event blocked by middleware"
        );
    }
}
