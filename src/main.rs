#![windows_subsystem = "windows"]
pub mod assets;
pub mod components;
pub mod config;
pub mod events;
pub mod pty;

use assets::Asset;
use components::app::App;
use config::font_helper::load_fonts;
use ipc::ipc_context::SHARED_ID;
use std::sync::atomic::Ordering;
use termio::cli::theme::theme_mgr::ThemeMgr;
use tmui::{
    application::Application, application_window::ApplicationWindow, graphics::icon::Icon,
    prelude::*,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[inline]
pub fn terminal_version() -> &'static str {
    VERSION
}

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

    ThemeMgr::loads::<Asset>("themes/builtin_themes.json");
    let icon = Asset::get("icons/icon.png").unwrap();
    let icon = unsafe { Icon::from_bytes(&icon.data) };

    let app = Application::builder()
        .width(1020)
        .height(600)
        .min_size((400, 200))
        .title("Termdot")
        .transparent(true)
        .defer_display(true)
        .decoration(false)
        .icon(icon)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    load_fonts();

    window.set_border_radius(10.);

    window.child(App::new());
}
