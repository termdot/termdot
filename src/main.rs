// #![windows_subsystem = "windows"]
use pty::termdot_pty::TermdotPty;
use termio::{cli::session::SessionPropsId, emulator::core::terminal_emulator::TerminalEmulator};
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
    window.child(terminal_emulator);

    window.register_run_after(move |win| {
        const ID: SessionPropsId = 0;

        if let Some(w) = win.find_id_mut(TerminalEmulator::id()) {
            let emulator = w.downcast_mut::<TerminalEmulator>().unwrap();
            emulator.start_custom_session(ID, TermdotPty::new());
            emulator.set_use_local_display(ID, true);
        }
    });
}
