use crate::assets::Asset;
use tmui::{
    icons::svg_icon::SvgIcon,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct NewTabButton {
    #[derivative(Default(value = "{
        let file = Asset::get(\"icons/add.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    add_tab: Tr<SvgIcon>,
}

impl ObjectSubclass for NewTabButton {
    const NAME: &'static str = "NewTabButton";
}

impl ObjectImpl for NewTabButton {
    fn initialize(&mut self) {
        self.set_valign(Align::Center);
        self.add_tab.width_request(25);
        self.add_tab.height_request(25);
    }
}

impl WidgetImpl for NewTabButton {}

impl NewTabButton {}
