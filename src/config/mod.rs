pub mod font_helper;

use std::cell::RefCell;
use termio::cli::theme::Theme;
use tmui::{
    font::Font,
    prelude::{Color, Derivative},
};

use crate::events::{EventBus, Events};

thread_local! {
    static CONFIG: RefCell<TermdotConfig> = RefCell::new(TermdotConfig::default());
}

const DEFAULT_FONT: [&str; 2] = ["Hack", "SimSun"];

#[derive(Derivative)]
#[derivative(Default)]
pub struct TermdotConfig {
    #[derivative(Default(value = "\"Dark\""))]
    default_theme: &'static str,
    current_theme: Option<Theme>,

    #[derivative(Default(value = "Color::rgb(18, 18, 18)"))]
    background: Color,
    #[derivative(Default(value = "Color::rgb(204, 204, 204)"))]
    foreground: Color,
    #[derivative(Default(value = "Color::rgb(64, 64, 64)"))]
    ctl_grey: Color,
    #[derivative(Default(value = "Color::rgb(245, 40, 40)"))]
    ctl_red: Color,
    #[derivative(Default(value = "Color::GREY_DARK"))]
    separator: Color,
    #[derivative(Default(value = "Color::hex(\"#3b78ff\")"))]
    active_session: Color,

    #[derivative(Default(value = "Font::with_families(&DEFAULT_FONT)"))]
    font: Font,
}

impl TermdotConfig {
    #[inline]
    pub fn set_theme(theme: Theme) {
        CONFIG.with(|config| {
            let mut config = config.borrow_mut();
            config.background = theme.background_color();
            config.foreground = theme.foreground_color();
            config.current_theme = Some(theme);
        });

        EventBus::push(Events::ThemeChanged);
    }

    #[inline]
    pub fn set_font(font: Font) {
        CONFIG.with(|config| {
            config.borrow_mut().font = font;
        });

        EventBus::push(Events::FontChanged);
    }

    #[inline]
    pub fn default_theme() -> &'static str {
        CONFIG.with(|config| config.borrow().default_theme)
    }

    #[inline]
    pub fn get_theme() -> Theme {
        CONFIG.with(|config| {
            config
                .borrow()
                .current_theme
                .clone()
                .expect("Fatal error, current theme is None.")
        })
    }

    #[inline]
    pub fn background() -> Color {
        CONFIG.with(|config| config.borrow().background)
    }

    #[inline]
    pub fn foreground() -> Color {
        CONFIG.with(|config| config.borrow().foreground)
    }

    #[inline]
    pub fn ctl_grey() -> Color {
        CONFIG.with(|config| config.borrow().ctl_grey)
    }

    #[inline]
    pub fn ctl_red() -> Color {
        CONFIG.with(|config| config.borrow().ctl_red)
    }

    #[inline]
    pub fn separator() -> Color {
        CONFIG.with(|config| config.borrow().separator)
    }

    #[inline]
    pub fn active_session() -> Color {
        CONFIG.with(|config| config.borrow().active_session)
    }

    #[inline]
    pub fn font() -> Font {
        CONFIG.with(|config| config.borrow().font.clone())
    }
}
