use super::{
    color_table::APP_BACKGROUND, session_tab::SessionTab, win_ctl_buttons::WinControlButtons,
};
use tlib::global_watch;
use tmui::{prelude::*, tlib::object::ObjectSubclass};

pub const TITLE_BAR_HEIGHT: i32 = 30;

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
#[global_watch(MouseMove)]
pub struct TitleBar {
    #[children]
    session_tab: Box<SessionTab>,
    #[children]
    win_control_buttons: Box<WinControlButtons>,

    pressed: bool,
    mouse_pos: Point,
}

impl ObjectSubclass for TitleBar {
    const NAME: &'static str = "TitleBar";
}

impl ObjectImpl for TitleBar {
    fn initialize(&mut self) {
        self.set_background(APP_BACKGROUND);
        self.height_request(TITLE_BAR_HEIGHT);
        self.set_hexpand(true);
        self.set_homogeneous(false);
        self.set_mouse_tracking(true);
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
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
