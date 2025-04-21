use tmui::tlib::{
    event_bus::event::{IEvent, IEventType},
    event_bus_init,
};

use crate::session::Session;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EventType {
    CreateSession = 0,
    HeartBeatUndetected,
    ShellExit,
    ShellReady,
    TitleChanged,

    ThemeChanged,
    FontChanged,
}
impl IEventType for EventType {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Events {
    CreateSession(Session),
    HeartBeatUndetected,
    ShellExit,
    ShellReay,
    TitleChanged(String),
    ThemeChanged,
    FontChanged,
}
impl IEvent for Events {
    type EventType = EventType;

    #[inline]
    fn ty(&self) -> EventType {
        match self {
            Self::CreateSession(..) => EventType::CreateSession,
            Self::HeartBeatUndetected => EventType::HeartBeatUndetected,
            Self::ShellExit => EventType::ShellExit,
            Self::ShellReay => EventType::ShellReady,
            Self::TitleChanged(..) => EventType::TitleChanged,
            Self::ThemeChanged => EventType::ThemeChanged,
            Self::FontChanged => EventType::FontChanged,
        }
    }
}

event_bus_init!(Events, EventType);
