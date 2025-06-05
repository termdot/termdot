use termio::{
    cli::{
        constant::ProtocolType, scheme::color_scheme_mgr::ColorSchemeMgr, session::SessionPropsId,
    },
    emulator::core::terminal_emulator::TerminalEmulator,
};
use tlib::event_bus::event_handle::EventHandle;
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use crate::{
    components::sessions::SessionTabTrait,
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
    pty::termdot_pty::TermdotPty,
};

use super::session_tab::SessionTab;

#[extends(Widget, Layout(HBox))]
pub struct SessionBar {
    active_session_id: Option<SessionPropsId>,
}

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

    #[allow(clippy::single_match)]
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

                    emulator.set_terminal_font(TermdotConfig::font());
                    TermdotConfig::set_theme(
                        ColorSchemeMgr::get(TermdotConfig::default_color_scheme()).unwrap(),
                    );

                    for c in self.children_mut() {
                        c.downcast_mut::<SessionTab>().unwrap().set_active(false);
                    }

                    let mut session_tab = SessionTab::new(session.ty);

                    session_tab.set_active(true);
                    session_tab.set_session_id(session.id);
                    self.active_session_id = Some(session.id);

                    connect!(
                        session_tab,
                        session_tab_clicked(),
                        self,
                        on_session_tab_clicked(ObjectId)
                    );
                    connect!(
                        session_tab,
                        close_icon_clicked(),
                        self,
                        on_close_icon_clicked(ObjectId, SessionPropsId)
                    );

                    self.add_child(session_tab);
                }
            }
            _ => {}
        }
    }
}

impl SessionBar {
    pub fn on_session_tab_clicked(&mut self, id: ObjectId) {
        let mut active_session_id = None;

        for c in self.children_mut() {
            let session_tab = c.downcast_mut::<SessionTab>().unwrap();
            if session_tab.id() == id {
                session_tab.set_active(true);

                active_session_id = Some(session_tab.get_session_id());
            } else {
                session_tab.set_active(false);
            }
        }

        if let Some(session_id) = active_session_id {
            if let Some(w) = self.window().find_id_mut(TerminalEmulator::id()) {
                let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
                emulator.switch_session(session_id);
            }

            self.active_session_id = Some(session_id);
        } else {
            self.active_session_id = None;
        }
    }

    pub fn on_close_icon_clicked(&mut self, id: ObjectId, session_id: SessionPropsId) {
        self.remove_children(id);

        if let Some(w) = self.window().find_id_mut(TerminalEmulator::id()) {
            let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
            emulator.remove_session(session_id);
            emulator.update();
        }

        if let Some(active_session_id) = self.active_session_id {
            if active_session_id == session_id {
                let mut children = self.children_mut();

                if children.is_empty() {
                    let session_tab = children[0].downcast_mut::<SessionTab>().unwrap();
                    session_tab.set_active(true);

                    if let Some(w) = ApplicationWindow::window().find_id_mut(TerminalEmulator::id())
                    {
                        let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
                        emulator.switch_session(session_tab.get_session_id());
                    }

                    self.active_session_id = Some(session_tab.get_session_id());
                } else {
                    self.active_session_id = None;
                }
            }
        }
    }
}
