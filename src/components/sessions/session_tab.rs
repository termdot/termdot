use crate::{
    components::title_bar::TITLE_BAR_HEIGHT,
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
};

use crate::assets::Asset;
use tlib::event_bus::event_handle::EventHandle;
use tmui::{
    icons::svg_icon::SvgIcon,
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct SessionTab {
    #[derivative(Default(value = "{
        let file = Asset::get(\"icons/godotengine.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    icon: Tr<SvgIcon>,

    #[children]
    session_label: Tr<Label>,
}

impl ObjectSubclass for SessionTab {
    const NAME: &'static str = "SessionTab";
}

impl ObjectImpl for SessionTab {
    fn initialize(&mut self) {
        EventBus::register(self);

        self.width_request(200);
        self.height_request(TITLE_BAR_HEIGHT - 1);
        self.set_margin_left(8);
        self.set_valign(Align::Center);
        self.set_homogeneous(false);
        self.set_strict_clip_widget(false);
        self.set_borders(0., 0., 2., 0.);

        self.set_border_color(TermdotConfig::active_session());
        self.set_background(TermdotConfig::background());

        let size = self.size();
        let icon_size = self.icon.size();
        let margin = 5;

        self.icon.set_valign(Align::Center);
        self.icon.set_halign(Align::Center);
        self.icon.set_margin_left(margin);

        self.session_label.set_size_hint(
            SizeHint::new().with_max_width(size.width() - icon_size.width() - margin * 2),
        );
        self.session_label.set_margin_left(margin);
        self.session_label.set_margin_top(2);
        self.session_label.set_halign(Align::Center);
        self.session_label.set_valign(Align::Center);
        self.session_label.set_content_halign(Align::Start);
        self.session_label.set_content_valign(Align::Center);
        self.session_label.set_text("SESSION TAB");
        self.session_label.set_color(TermdotConfig::foreground());
        self.session_label.set_auto_wrap(false);
        self.session_label.set_font(TermdotConfig::font());
    }

    fn on_drop(&mut self) {
        EventBus::remove(self);
    }
}

impl WidgetImpl for SessionTab {}

impl SessionTab {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }

    #[inline]
    pub fn set_session_name(&mut self, name: &str) {
        self.session_label.set_text(name);
    }
}

impl EventHandle for SessionTab {
    type EventType = EventType;
    type Event = Events;

    #[inline]
    fn listen(&self) -> Vec<EventType> {
        vec![
            EventType::TitleChanged,
            EventType::ThemeChanged,
            EventType::FontChanged,
        ]
    }

    #[inline]
    fn handle(&mut self, evt: &Events) {
        match evt {
            Events::TitleChanged(title) => {
                self.session_label.set_text(title);
            }

            Events::ThemeChanged => {
                self.set_border_color(TermdotConfig::active_session());
                self.set_background(TermdotConfig::background());
                self.session_label.set_color(TermdotConfig::foreground());
            }

            Events::FontChanged => {
                self.session_label.set_font(TermdotConfig::font());
            }

            _ => {}
        }
    }
}
