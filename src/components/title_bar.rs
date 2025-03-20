use super::{
    color_table::APP_BACKGROUND, session_tab::SessionTab, win_ctl_buttons::WinControlButtons,
};
use tmui::{prelude::*, tlib::object::ObjectSubclass};

pub const TITLE_BAR_HEIGHT: i32 = 30;

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct TitleBar {
    #[children]
    session_tab: Box<SessionTab>,
    #[children]
    win_control_buttons: Box<WinControlButtons>,
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
    }
}

impl WidgetImpl for TitleBar {}

impl TitleBar {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
