use crate::{
    components::title_bar::TITLE_BAR_HEIGHT,
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
};

use crate::assets::Asset;
use termio::cli::{constant::ProtocolType, session::SessionPropsId};
use tlib::{event_bus::event_handle::EventHandle, signals};
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

    session_id: SessionPropsId,
}

pub trait SessionTabTrait: ActionExt {
    signals!(
        SessionTab:

        /// Emit when session tab has been mouse released.
        ///
        /// @params:
        /// @id: The id of SessionTab.
        session_tab_clicked(ObjectId);
    );
}
impl SessionTabTrait for SessionTab {}

impl ObjectSubclass for SessionTab {
    const NAME: &'static str = "SessionTab";
}

impl ObjectImpl for SessionTab {
    fn initialize(&mut self) {
        EventBus::register(self);

        self.width_request(200);
        self.height_request(TITLE_BAR_HEIGHT - 1);
        self.set_valign(Align::Center);
        self.set_homogeneous(false);
        self.set_strict_clip_widget(false);
        self.set_borders(0., 0., 2., 0.);
        self.enable_bubble(EventBubble::MOUSE_RELEASED);

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
        self.session_label.set_color(TermdotConfig::foreground());
        self.session_label.set_auto_wrap(false);
        self.session_label.set_font(TermdotConfig::font());
    }

    fn on_drop(&mut self) {
        EventBus::remove(self);
    }
}

impl WidgetImpl for SessionTab {
    #[inline]
    fn on_mouse_released(&mut self, _: &MouseEvent) {
        emit!(self, session_tab_clicked(self.id()));
    }
}

impl SessionTab {
    #[inline]
    pub fn new(protocol_type: ProtocolType) -> Tr<Self> {
        let mut tab = Self::new_alloc();

        match protocol_type {
            ProtocolType::Custom => {
                let file = Asset::get("icons/godotengine.svg").unwrap();
                tab.icon.load_bytes(file.data.as_ref());
            }
            ProtocolType::Cmd => {
                let file = Asset::get("icons/cmd.svg").unwrap();
                tab.icon.load_bytes(file.data.as_ref());

                tab.set_session_name("cmd.exe")
            }
            ProtocolType::PowerShell => {
                let file = Asset::get("icons/powershell.svg").unwrap();
                tab.icon.load_bytes(file.data.as_ref());

                tab.set_session_name("Windows PowerShell")
            }
            _ => {}
        }

        tab
    }

    #[inline]
    pub fn set_session_name(&mut self, name: &str) {
        self.session_label.set_text(name);
    }

    #[inline]
    pub fn set_session_id(&mut self, id: SessionPropsId) {
        self.session_id = id;
    }

    #[inline]
    pub fn get_session_id(&self) -> SessionPropsId {
        self.session_id
    }

    #[inline]
    pub fn set_active(&mut self, active: bool) {
        if active {
            self.set_border_color(TermdotConfig::active_session());
        } else {
            self.set_border_color(TermdotConfig::background());
        }
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
