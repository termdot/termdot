use super::color_table::{
    SESSION_ALIVE_COLOR, SESSION_DEAD_COLOR, TERMINAL_BACKGROUND, TERMINAL_FOREGROUND,
};
use tlib::skia_safe::Path;
use tmui::{
    graphics::box_shadow::{BoxShadow, ShadowSide},
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[derive(Childable)]
pub struct SessionTab {
    #[child]
    session_label: Box<Label>,

    session_alive: bool,
}

impl ObjectSubclass for SessionTab {
    const NAME: &'static str = "SessionTab";
}

impl ObjectImpl for SessionTab {
    fn initialize(&mut self) {
        self.width_request(200);
        self.height_request(25);
        self.set_margin_left(8);
        self.set_valign(Align::End);
        self.set_border_radius_sep(5., 5., 0., 0.);
        self.set_box_shadow(BoxShadow::new(
            5.,
            Color::BLACK,
            None,
            Some(ShadowSide::new(&[
                ShadowSide::LEFT,
                ShadowSide::TOP,
                ShadowSide::RIGHT,
            ])),
            Some(2),
            None,
        ));
        self.set_strict_clip_widget(false);

        self.set_background(TERMINAL_BACKGROUND);

        self.session_label.set_halign(Align::Center);
        self.session_label.set_valign(Align::Center);
        self.session_label.set_content_halign(Align::Center);
        self.session_label.set_content_valign(Align::Center);
        self.session_label.set_text("SESSION TAB");
        self.session_label.set_color(TERMINAL_FOREGROUND);
    }
}

impl WidgetImpl for SessionTab {
    fn paint(&mut self, painter: &mut Painter) {
        let rect = self.rect_f();
        let tl = rect.top_left();
        let bl = rect.bottom_left();
        let p = (tl.x() + 10., (tl.y() + bl.y()) / 2. - 1.);
        let color = if self.session_alive {
            SESSION_ALIVE_COLOR
        } else {
            SESSION_DEAD_COLOR
        };

        painter.set_color(color);
        painter.set_antialiasing(true);
        let mut path = Path::default();
        path.add_circle(p, 3., None);
        painter.draw_path(&path);

        let r = 7.;
        painter.set_color(TERMINAL_BACKGROUND);

        let lb = FRect::new(rect.left() - r, rect.bottom() - r, r, r);
        painter.draw_varying_arc_global(lb, 0., 90., 2., 2., 8);
        let rb = FRect::new(rect.right(), rect.bottom() - r, r, r);
        painter.draw_varying_arc_global(rb, 90., 90., 2., 2., 8);
    }
}

impl SessionTab {
    #[inline]
    pub fn set_session_name(&mut self, name: &str) {
        self.session_label.set_text(name);
    }

    #[inline]
    pub fn set_session_alive(&mut self, alive: bool) {
        self.session_alive = alive;
        self.update();
    }
}
