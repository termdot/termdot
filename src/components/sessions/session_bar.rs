use termio::{
    cli::{constant::ProtocolType, scheme::color_scheme_mgr::ColorSchemeMgr},
    emulator::core::terminal_emulator::{TerminalEmulator, TerminalEmulatorTrait},
};
use tlib::{event_bus::event_handle::EventHandle, iter_executor, log::warn};
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{IterExecutor, WidgetImpl},
};

use crate::{
    components::{sessions::SessionTabTrait, title_bar::TitleBar},
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
    pty::termdot_pty::TermdotPty,
};

use super::{session_tab::SessionTab, MAX_WIDTH, MIN_WIDTH};

#[extends(Widget, Layout(HBox))]
#[iter_executor]
pub struct SessionBar {
    active_session_panel_id: Option<ObjectId>,
    removed_session_panel: Option<ObjectId>,
}

impl ObjectSubclass for SessionBar {
    const NAME: &'static str = "SessionBar";
}

impl ObjectImpl for SessionBar {
    fn initialize(&mut self) {
        EventBus::register(self);

        self.set_vexpand(true);
        self.set_margin_left(20);

        if let Some(w) = ApplicationWindow::window().find_id_mut(TerminalEmulator::id()) {
            let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
            connect!(
                emulator,
                session_panel_finished(),
                self,
                on_session_panel_finished(ObjectId)
            );
        } else {
            warn!("[SessionBar::initialize] The `TerminalEmulator` is None.");
        }
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
        vec![
            EventType::CreateSession,
            EventType::ShellReady,
            EventType::TitleChanged,
        ]
    }

    #[allow(clippy::single_match)]
    #[inline]
    fn handle_evt(&mut self, evt: &Self::Event) {
        match evt {
            Events::CreateSession(session) => {
                let win = self.window();

                if let Some(w) = win.find_id_mut(TerminalEmulator::id()) {
                    let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();

                    let session_panel_id = match session.ty {
                        ProtocolType::Custom => {
                            emulator.start_custom_session(session.id, TermdotPty::new())
                        }
                        _ => emulator.start_session(session.id, session.ty),
                    };

                    emulator.set_terminal_font(TermdotConfig::font());
                    TermdotConfig::set_theme(
                        ColorSchemeMgr::get(TermdotConfig::default_color_scheme()).unwrap(),
                    );

                    for c in self.children_mut() {
                        c.downcast_mut::<SessionTab>().unwrap().set_active(false);
                    }

                    let mut session_tab = SessionTab::new(session.ty);

                    session_tab.set_active(true);
                    session_tab.add_session_id(session.id);
                    session_tab.set_active_session_id(session.id);
                    session_tab.set_session_panel_id(session_panel_id);
                    self.active_session_panel_id = Some(session_panel_id);

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
                        on_close_icon_clicked(ObjectId)
                    );

                    self.add_child(session_tab);

                    self.calc_session_tab();
                }
            }

            Events::ShellReay(..) => {
                // Do nothing temporary
            }

            Events::TitleChanged(session_id, title) => {
                for child in self.children_mut() {
                    let session_tab = child
                        .downcast_mut::<SessionTab>()
                        .expect("[SessionBar::handle::TitleChanged] downcast_mut is None.");
                    if session_tab.contains_session_id(*session_id) {
                        session_tab.set_title(title);
                        break;
                    }
                }
            }

            _ => {}
        }
    }
}

impl SessionBar {
    pub fn on_session_tab_clicked(&mut self, id: ObjectId) {
        let mut active_session_panel_id = None;
        for c in self.children_mut() {
            let session_tab = c.downcast_mut::<SessionTab>().unwrap();
            if session_tab.id() == id {
                session_tab.set_active(true);

                if let Some(w) = ApplicationWindow::window().find_id_mut(TerminalEmulator::id()) {
                    let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
                    emulator.switch_session(session_tab.get_active_session_id());
                }

                active_session_panel_id = Some(session_tab.get_session_panel_id());
            } else {
                session_tab.set_active(false);
            }
        }
        self.active_session_panel_id = active_session_panel_id;
    }

    pub fn on_close_icon_clicked(&mut self, session_tab_id: ObjectId) {
        if let Some(w) = self.window().find_id_mut(TerminalEmulator::id()) {
            let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();

            for c in self.children().iter() {
                if c.id() == session_tab_id {
                    let session_tab = c.downcast_ref::<SessionTab>().expect(
                        "[SessionBar::on_close_icon_clicked] Downcast widget to SessionTab failed.",
                    );
                    for session_id in session_tab.session_id_iter() {
                        emulator.remove_session(*session_id);
                    }
                    break;
                }
            }
        }
    }

    pub fn calc_session_tab(&mut self) {
        let count = self.children().len() as i32;
        if count == 0 {
            return;
        }

        let parent = self
            .get_parent_ref()
            .expect("[SessionBar::on_session_count_changed] get parent is None.")
            .downcast_ref::<TitleBar>()
            .expect("[SessionBar::on_session_count_changed] Cast to `TitleBar` failed.");
        let theo_width = parent.get_title_bar_theoretical_width();
        let width = (theo_width / count).clamp(MIN_WIDTH, MAX_WIDTH);

        let mut children = self.children_mut();
        Widget::resize_batch(children[0].to_tr(), &mut children, Some(width), None);
    }

    fn on_session_panel_finished(&mut self, panel_id: ObjectId) {
        for child in self.children() {
            let session_tab = child.downcast_ref::<SessionTab>().unwrap();
            if session_tab.get_session_panel_id() == panel_id {
                self.remove_session_tab(session_tab.id(), panel_id);
                break;
            }
        }
    }

    fn remove_session_tab(&mut self, session_tab_id: ObjectId, session_panel_id: ObjectId) {
        self.remove_children(session_tab_id);
        self.removed_session_panel = Some(session_panel_id);

        self.calc_session_tab();
    }
}

impl IterExecutor for SessionBar {
    #[inline]
    fn iter_execute(&mut self) {
        if let Some(session_panel_id) = self.removed_session_panel.take() {
            if let Some(active_session_panel_id) = self.active_session_panel_id {
                if active_session_panel_id == session_panel_id {
                    let mut children = self.children_mut();

                    if !children.is_empty() {
                        let session_tab = children[0].downcast_mut::<SessionTab>().unwrap();
                        session_tab.set_active(true);

                        if let Some(w) =
                            ApplicationWindow::window().find_id_mut(TerminalEmulator::id())
                        {
                            let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
                            emulator.switch_session(session_tab.get_active_session_id());
                        }

                        self.active_session_panel_id = Some(session_tab.get_session_panel_id());
                    } else {
                        self.active_session_panel_id = None;
                    }
                }

                ApplicationWindow::window().layout_change(self);
            }
        }
    }
}
