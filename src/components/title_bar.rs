use super::{
    sessions::{new_tab_button::NewTabButton, SessionBar},
    win_ctl_buttons::WinControlButtons,
};
use crate::{
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
};
use tlib::{event_bus::event_handle::EventHandle, global_watch};
use tmui::{prelude::*, tlib::object::ObjectSubclass};

pub const TITLE_BAR_HEIGHT: i32 = 35;

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
#[global_watch(MouseMove)]
pub struct TitleBar {
    #[children]
    session_bar: Tr<SessionBar>,
    #[children]
    win_control_buttons: Tr<WinControlButtons>,
    #[children]
    new_tab_button: Tr<NewTabButton>,

    pressed: bool,
    mouse_pos: Point,
}

impl ObjectSubclass for TitleBar {
    const NAME: &'static str = "TitleBar";
}

impl ObjectImpl for TitleBar {
    fn initialize(&mut self) {
        EventBus::register(self);

        self.set_background(TermdotConfig::background());
        self.height_request(TITLE_BAR_HEIGHT);
        self.set_hexpand(true);
        self.set_homogeneous(false);
        self.set_mouse_tracking(true);

        self.set_borders(0., 0., 1., 0.);
        self.set_border_color(TermdotConfig::separator());

        self.enable_bubble(EventBubble::MOUSE_PRESSED);
        self.enable_bubble(EventBubble::MOUSE_RELEASED);

        connect!(self, size_changed(), self, on_size_changed(Size));
    }

    fn on_drop(&mut self) {
        EventBus::remove(self);
    }
}

impl GlobalWatchImpl for TitleBar {
    fn on_global_mouse_move(&mut self, evt: &MouseEvent) -> bool {
        if self.pressed {
            self.on_mouse_move(evt);
            true
        } else {
            false
        }
    }
}

impl WidgetImpl for TitleBar {
    fn on_mouse_pressed(&mut self, evt: &MouseEvent) {
        self.pressed = true;
        let window = self.window();
        self.mouse_pos = window.map_to_outer(&self.map_to_global(&evt.position().into()));
    }

    fn on_mouse_released(&mut self, _: &MouseEvent) {
        self.pressed = false;
    }

    fn on_mouse_move(&mut self, event: &MouseEvent) {
        if self.pressed {
            let window = ApplicationWindow::window();
            let outer_position = window.map_to_outer(&self.map_to_global(&event.position().into()));
            let offset = outer_position - self.mouse_pos;
            if offset == Point::new(0, 0) {
                return;
            }
            self.mouse_pos = outer_position;

            let win_pos = window.outer_position();
            window.request_win_position(win_pos + offset);
        }
    }
}

impl TitleBar {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }

    #[inline]
    pub fn on_size_changed(&mut self, _: Size) {
        let session_bar_width = self.get_title_bar_theoretical_width();

        self.session_bar
            .set_size_hint(SizeHint::new().with_max_width(session_bar_width));

        self.session_bar.calc_session_tab();
    }

    #[inline]
    pub fn get_title_bar_theoretical_width(&self) -> i32 {
        self.size().width()
            - self.new_tab_button.get_width_request()
            - self.win_control_buttons.get_width_request()
            // Session bar margin left
            - 20
            // New tab button margin left
            - 5
    }
}

impl EventHandle for TitleBar {
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
                self.set_background(TermdotConfig::background());
                self.set_border_color(TermdotConfig::separator());
            }
            _ => {}
        }
    }
}
