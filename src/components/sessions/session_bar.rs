use termio::{
    cli::{constant::ProtocolType, scheme::color_scheme_mgr::ColorSchemeMgr},
    emulator::core::terminal_emulator::TerminalEmulator,
};
use tlib::event_bus::event_handle::EventHandle;
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use crate::{
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
    pty::termdot_pty::TermdotPty,
};

use super::session_tab::SessionTab;

#[extends(Widget, Layout(HBox))]
pub struct SessionBar {}

impl ObjectSubclass for SessionBar {
    const NAME: &'static str = "SessionBar";
}

impl ObjectImpl for SessionBar {
    fn initialize(&mut self) {
        EventBus::register(self);

        self.set_margin_left(8);
        self.set_vexpand(true);
        self.set_margin_left(20);
    }

    fn on_drop(&mut self) {
        EventBus::remove(self);
    }
}

impl WidgetImpl for SessionBar {}

impl EventHandle for SessionBar {
    type EventType = EventType;
    type Event = Events;

    #[inline]
    fn listen(&self) -> Vec<Self::EventType> {
        vec![EventType::CreateSession]
    }

    #[inline]
    fn handle(&mut self, evt: &Self::Event) {
        match evt {
            Events::CreateSession(session) => {
                let win = self.window();

                if let Some(w) = win.find_id_mut(TerminalEmulator::id()) {
                    let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();

                    match session.ty {
                        ProtocolType::Custom => {
                            emulator.start_custom_session(session.id, TermdotPty::new())
                        }
                        _ => {
                            emulator.start_session(session.id, session.ty);
                        }
                    }

                    TermdotConfig::set_theme(
                        ColorSchemeMgr::get(TermdotConfig::default_color_scheme()).unwrap(),
                    );
                    emulator.set_terminal_font(TermdotConfig::font());

                    self.add_child(SessionTab::new(session.ty));
                }
            }
            _ => {}
        }
    }
}
