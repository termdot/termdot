// #![windows_subsystem = "windows"]
pub mod assets;
pub mod components;

use components::{color_table::APP_BACKGROUND, title_bar::TitleBar};
use ipc::ipc_context::SHARED_ID;
use pty::termdot_pty::TermdotPty;
use std::sync::atomic::Ordering;
use termio::{cli::session::SessionPropsId, emulator::core::terminal_emulator::TerminalEmulator};
use tmui::{application::Application, application_window::ApplicationWindow, prelude::*};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let id = args[1].parse::<u64>().unwrap();
        SHARED_ID.store(id, Ordering::Release);
    }

    #[cfg(debug_assertions)]
    {
        log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();
    }

    let app = Application::builder()
        .width(1020)
        .height(600)
        .resizable(false)
        .title("Termdot")
        .transparent(true)
        .defer_display(true)
        .decoration(false)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.set_background(APP_BACKGROUND);
    let title_bar = TitleBar::new();
    let mut terminal_emulator = TerminalEmulator::new();
    terminal_emulator.set_hexpand(true);
    terminal_emulator.set_vexpand(true);

    let mut vbox = VBox::new();
    vbox.set_vexpand(true);
    vbox.set_hexpand(true);
    vbox.add_child(title_bar);
    vbox.add_child(terminal_emulator);

    window.child(vbox);

    window.register_run_after(move |win| {
        const ID: SessionPropsId = 0;

        if let Some(w) = win.find_id_mut(TerminalEmulator::id()) {
            let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
            emulator.start_custom_session(ID, TermdotPty::new());
            emulator.set_use_local_display(ID, true);
        }
    });
}
