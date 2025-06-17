use termio::cli::session::SessionPropsId;
use tmui::tlib::{
    event_bus::event::{IEvent, IEventType},
    event_bus_init,
};

use crate::session::Session;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EventType {
    CreateSession = 0,
    ShellExit,
    ShellReady,
    TitleChanged,
    ThemeChanged,
    FontChanged,
    SessionDropdownListHide,
}
impl IEventType for EventType {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Events {
    CreateSession(Session),
    ShellExit,
    ShellReay(SessionPropsId),
    TitleChanged(SessionPropsId, String),
    ThemeChanged,
    FontChanged,
    SessionDropdownListHide,
}
impl IEvent for Events {
    type EventType = EventType;

    #[inline]
    fn ty(&self) -> EventType {
        match self {
            Self::CreateSession(..) => EventType::CreateSession,
            Self::ShellExit => EventType::ShellExit,
            Self::ShellReay(..) => EventType::ShellReady,
            Self::TitleChanged(..) => EventType::TitleChanged,
            Self::ThemeChanged => EventType::ThemeChanged,
            Self::FontChanged => EventType::FontChanged,
            Self::SessionDropdownListHide => EventType::SessionDropdownListHide,
        }
    }
}

event_bus_init!(Events, EventType);
