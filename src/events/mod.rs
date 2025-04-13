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
    MasterExit,
    MasterReady,
    TitleChanged,

    ThemeChanged,
    FontChanged,
}
impl IEventType for EventType {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Events {
    CreateSession(Session),
    HeartBeatUndetected,
    MasterExit,
    MasterReay,
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
            Self::MasterExit => EventType::MasterExit,
            Self::MasterReay => EventType::MasterReady,
            Self::TitleChanged(..) => EventType::TitleChanged,
            Self::ThemeChanged => EventType::ThemeChanged,
            Self::FontChanged => EventType::FontChanged,
        }
    }
}

event_bus_init!(Events, EventType);
