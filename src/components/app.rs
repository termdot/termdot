use super::title_bar::TitleBar;
use crate::{event_bus::EventBus, pty::termdot_pty::TermdotPty};
use termio::{cli::session::SessionPropsId, emulator::core::terminal_emulator::TerminalEmulator};
use tlib::{iter_executor, run_after};
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{IterExecutor, WidgetImpl},
};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
#[run_after]
#[iter_executor]
pub struct App {
    #[children]
    title_bar: Box<TitleBar>,
    #[children]
    terminal_emulator: Box<TerminalEmulator>,
}

impl ObjectSubclass for App {
    const NAME: &'static str = "App";
}

impl ObjectImpl for App {
    fn initialize(&mut self) {
        self.terminal_emulator.set_hexpand(true);
        self.terminal_emulator.set_vexpand(true);

        self.set_vexpand(true);
        self.set_hexpand(true);
    }
}

impl WidgetImpl for App {
    fn run_after(&mut self) {
        const ID: SessionPropsId = 0;
        let win = self.window();

        if let Some(w) = win.find_id_mut(TerminalEmulator::id()) {
            let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
            emulator.start_custom_session(ID, TermdotPty::new());
        }
    }
}

impl IterExecutor for App {
    #[inline]
    fn iter_execute(&mut self) {
        EventBus::process_deferred_evts();
    }
}

impl App {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
