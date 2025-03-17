// #![windows_subsystem = "windows"]
use pty::termdot_pty::TermdotPty;
use termio::emulator::core::terminal_emulator::TerminalEmulator;
use tmui::{application::Application, application_window::ApplicationWindow, prelude::*};

fn main() {
    log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1020)
        .height(600)
        .title("Termdot")
        .transparent(true)
        .defer_display(true)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let terminal_emulator = TerminalEmulator::new();
    let id = terminal_emulator.id();
    window.child(terminal_emulator);

    window.register_run_after(move |win| {
        if let Some(w) = win.find_id_mut(id) {
            let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
            emulator.start_custom_session(0, TermdotPty::new());
            emulator.set_use_local_display(0, true);
        }
    });
}
