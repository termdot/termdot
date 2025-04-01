#![windows_subsystem = "windows"]
pub mod assets;
pub mod components;
pub mod events;
pub mod pty;

use components::{app::App, color_table::APP_BACKGROUND};
use ipc::ipc_context::SHARED_ID;
use std::sync::atomic::Ordering;
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
        .min_size((400, 200))
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

    window.child(App::new());
}
