pub mod select_option;

use crate::{
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
    session::Session,
};
use select_option::SelectOption;
use termio::cli::constant::ProtocolType;
use tlib::log;
use tlib::{event_bus::event_handle::EventHandle, utils::SnowflakeGuidGenerator};
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::list_view::ListView,
    widget::{widget_ext::FocusStrat, WidgetImpl},
};

const MAX_VISIBLE_ITEMS: i32 = 20;
const MINIMUN_WIDTH: i32 = 300;
const MINIMUN_HEIGHT: i32 = 50;

#[extends(Popup)]
#[derive(Childable)]
#[tlib::win_widget(
    o2s(SessionDropdownMsg),
    s2o(SessionDropdownMsg),
    PopupImpl(calculate_position(popup_position_calculate))
)]
pub struct SessionDropdownList {
    #[child]
    list: Tr<ListView>,
}

impl ObjectSubclass for SessionDropdownList {
    const NAME: &'static str = "SessionDropdownList";
}

impl ObjectImpl for SessionDropdownList {
    fn initialize(&mut self) {
        EventBus::register(self);

        self.width_request(MINIMUN_WIDTH);
        self.height_request(MINIMUN_HEIGHT + 10 * 2);
        self.set_paddings(10, 10, 10, 10);
        self.set_border_radius(10.);
        self.set_borders(1., 1., 1., 1.);
        self.set_background(TermdotConfig::popup_background());
        self.set_border_color(TermdotConfig::pre_hover());

        self.list.set_reset_effect_node_on_hide(true);
        self.list.set_line_height(16);
        self.list.set_layout_mode(LayoutMode::Overlay);
        self.list.set_hexpand(true);
        self.list.set_vexpand(true);
        self.list.set_font(TermdotConfig::font());
        self.list.register_node_released(|node, _, _| {
            let val = node.get_value::<ProtocolType>(0).unwrap();
            let dropdown_list = node
                .get_view()
                .get_parent_mut()
                .unwrap()
                .downcast_mut::<SessionDropdownList>()
                .unwrap();

            dropdown_list.on_list_value_changed(val);

            dropdown_list.trans_focus_take(FocusStrat::Restore);

            dropdown_list.hide();
        });

        self.list.add_node(&SelectOption::new(ProtocolType::Cmd));
        self.list
            .add_node(&SelectOption::new(ProtocolType::PowerShell));

        self.calc_height();

        let scroll_bar = self.list.scroll_bar_mut();
        scroll_bar.set_visible_in_valid(true);
    }

    fn on_drop(&mut self) {
        EventBus::remove(self);
    }
}

impl WidgetImpl for SessionDropdownList {}

// impl PopupImpl for SessionDropdownList {
//     #[inline]
//     fn calculate_position(&self, base_rect: Rect, _: Point) -> Point {
//         base_rect.bottom_left()
//     }
// }
fn popup_position_calculate(_: &dyn WidgetImpl, base_rect: Rect, _: Point) -> Point {
    base_rect.bottom_left()
}

impl SessionDropdownList {
    #[inline]
    pub fn trans_focus_take(&mut self, strat: FocusStrat) {
        self.list.take_over_focus(strat);
    }

    #[inline]
    fn calc_height(&mut self) {
        let len = (self.list.len() as i32).min(MAX_VISIBLE_ITEMS);
        if len == 0 {
            self.height_request(self.list.get_line_height());
        } else {
            let height = (self.list.get_line_height() + self.list.get_line_spacing()) * len;

            // Add the height of borders.
            self.height_request(height + 10 * 2)
        }
        ApplicationWindow::window().layout_change(self);
    }

    #[inline]
    fn on_list_value_changed(&mut self, protocol_type: ProtocolType) {
        self.send_cross_win_msg(SessionDropdownMsg::CreateSession(protocol_type));
    }
}

impl EventHandle for SessionDropdownList {
    type EventType = EventType;
    type Event = Events;

    #[inline]
    fn listen(&self) -> Vec<Self::EventType> {
        vec![EventType::ThemeChanged]
    }

    #[inline]
    #[allow(clippy::single_match)]
    fn handle_evt(&mut self, evt: &Self::Event) {
        match evt {
            Events::ThemeChanged => {
                self.set_background(TermdotConfig::popup_background());
                self.set_border_color(TermdotConfig::pre_hover());
                self.list.set_background(TermdotConfig::popup_background());
                self.list.set_font(TermdotConfig::font());
            }

            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionDropdownMsg {
    CreateSession(ProtocolType),
}

impl CrossWinMsgHandler for SessionDropdownList {
    type T = SessionDropdownMsg;

    #[inline]
    fn handle(&mut self, _msg: Self::T) {}
}

impl CrossWinMsgHandler for CorrSessionDropdownList {
    type T = SessionDropdownMsg;

    #[inline]
    fn handle(&mut self, msg: Self::T) {
        match msg {
            SessionDropdownMsg::CreateSession(protocol_type) => {
                let session =
                    Session::new(SnowflakeGuidGenerator::next_id().unwrap(), protocol_type);
                EventBus::push(Events::CreateSession(session));
            }
        }
    }
}
