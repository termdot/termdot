pub mod font_helper;

use std::cell::RefCell;
use termio::cli::scheme::ColorScheme;
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
    default_color_scheme: &'static str,
    current_color_scheme: Option<ColorScheme>,

    #[derivative(Default(value = "Color::rgb(18, 18, 18)"))]
    background: Color,
    #[derivative(Default(value = "Color::rgb(42, 42, 42)"))]
    popup_background: Color,
    #[derivative(Default(value = "Color::rgb(204, 204, 204)"))]
    foreground: Color,
    #[derivative(Default(value = "Color::rgb(32, 32, 32)"))]
    pre_hover: Color,
    #[derivative(Default(value = "Color::rgb(64, 64, 64)"))]
    hover: Color,
    #[derivative(Default(value = "Color::rgb(245, 40, 40)"))]
    error: Color,
    #[derivative(Default(value = "Color::rgb(64, 64, 64)"))]
    separator: Color,
    #[derivative(Default(value = "Color::hex(\"#3b78ff\")"))]
    active_session: Color,
    #[derivative(Default(value = "Color::hex(\"#0f6682\")"))]
    highlight: Color,
    #[derivative(Default(value = "Color::hex(\"#19aad8\")"))]
    selection: Color,

    #[derivative(Default(value = "Font::with_families(&DEFAULT_FONT)"))]
    font: Font,
}

impl TermdotConfig {
    #[inline]
    pub fn set_theme(theme: ColorScheme) {
        CONFIG.with(|config| {
            let mut config = config.borrow_mut();
            config.background = theme.background_color();
            config.foreground = theme.foreground_color();
            config.current_color_scheme = Some(theme);
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
    pub fn default_color_scheme() -> &'static str {
        CONFIG.with(|config| config.borrow().default_color_scheme)
    }

    #[inline]
    pub fn get_color_scheme() -> ColorScheme {
        CONFIG.with(|config| {
            config
                .borrow()
                .current_color_scheme
                .clone()
                .expect("Fatal error, current theme is None.")
        })
    }

    #[inline]
    pub fn background() -> Color {
        CONFIG.with(|config| config.borrow().background)
    }

    #[inline]
    pub fn popup_background() -> Color {
        CONFIG.with(|config| config.borrow().popup_background)
    }

    #[inline]
    pub fn foreground() -> Color {
        CONFIG.with(|config| config.borrow().foreground)
    }

    #[inline]
    pub fn pre_hover() -> Color {
        CONFIG.with(|config| config.borrow().pre_hover)
    }

    #[inline]
    pub fn hover() -> Color {
        CONFIG.with(|config| config.borrow().hover)
    }

    #[inline]
    pub fn error() -> Color {
        CONFIG.with(|config| config.borrow().error)
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
    pub fn highlight() -> Color {
        CONFIG.with(|config| config.borrow().highlight)
    }

    #[inline]
    pub fn selection() -> Color {
        CONFIG.with(|config| config.borrow().selection)
    }

    #[inline]
    pub fn font() -> Font {
        CONFIG.with(|config| config.borrow().font.clone())
    }
}
