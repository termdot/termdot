// #![windows_subsystem = "windows"]
pub mod assets;
pub mod components;
pub mod config;
pub mod events;
pub mod pty;
pub mod session;

use assets::Asset;
use components::app::App;
use config::{font_helper::load_fonts, TermdotConfig};
use termio::cli::scheme::color_scheme_mgr::ColorSchemeMgr;
use tlib::log::error;
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
    #[cfg(debug_assertions)]
    {
        log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();
    }

    set_panic_hook();

    ColorSchemeMgr::loads::<Asset>("themes/builtin_themes.json");
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
