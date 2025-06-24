#![windows_subsystem = "windows"]
pub mod assets;
pub mod components;
pub mod config;
pub mod events;
pub mod pty;
pub mod session;

use std::sync::atomic::{AtomicU64, Ordering};

use assets::Asset;
use common::typedef::RegisterInfoId;
use components::app::App;
use config::{font_helper::load_fonts, TermdotConfig};
use termio::cli::scheme::color_scheme_mgr::ColorSchemeMgr;
use tlib::{log::error, utils::SnowflakeGuidGenerator};
use tmui::{
    application::Application, application_window::ApplicationWindow, graphics::icon::Icon,
    prelude::*,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

static TERMINAL_ID: AtomicU64 = AtomicU64::new(0);

#[inline]
pub fn terminal_version() -> &'static str {
    VERSION
}

fn main() {
    #[cfg(debug_assertions)]
    {
        log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();
    }

    set_panic_hook();

    ColorSchemeMgr::loads::<Asset>("themes/builtin_themes.json");
    let icon = Asset::get("icons/icon.png").unwrap();
    let icon = unsafe { Icon::from_bytes(&icon.data) };

    load_fonts();
    set_terminal_id();

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
    window.set_background(TermdotConfig::background());

    window.set_border_radius(8.);

    window.child(App::new());
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let thread = std::thread::current();
        let thread_name = thread.name().unwrap_or("unnamed");
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic"
        };

        let location = panic_info.location();
        let panic = format!(
            "[PANIC] [{}] Panic occurred.\r\nLocation: {:?}\r\nPanic:\r\n{}\r\n",
            thread_name, location, msg
        );
        error!("{}", panic);
        common::log::LocalLog::append(panic);
    }));
}

#[inline]
fn set_terminal_id() {
    let id =
        SnowflakeGuidGenerator::next_id().expect("[Main::set_terminal_id] Generate guid failed.");
    TERMINAL_ID.store(id, Ordering::Release);
}

#[inline]
pub fn terminal_id() -> RegisterInfoId {
    TERMINAL_ID.load(Ordering::Relaxed)
}
