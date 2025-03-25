use super::events::{EventType, Events};

pub trait EventHandle {
    fn listen(&self) -> Vec<EventType>;

    fn handle(&mut self, evt: &Events);
}
