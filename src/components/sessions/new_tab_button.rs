use std::{cell::Cell, rc::Rc};

use crate::{
    assets::Asset,
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
};
use tlib::{event_bus::event_handle::EventHandle, namespace::MouseButton};
use tmui::{
    icons::svg_icon::SvgIcon,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{callbacks::CallbacksRegister, WidgetImpl},
};

use super::dropdown_list::SessionDropdownList;

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
#[popupable]
pub struct NewTabButton {
    #[derivative(Default(value = "{
        let file = Asset::get(\"icons/add.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    add_tab: Tr<SvgIcon>,

    #[derivative(Default(value = "{
        let file = Asset::get(\"icons/drop-down.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    drop_down: Tr<SvgIcon>,

    dropdown_list_showed: Rc<Cell<bool>>,
    mouse_in_addtab_icon: bool,
    mouse_in_dropdown_icon: bool,
    /// Dropdown list just hide.
    just_hide: bool,
}

impl ObjectSubclass for NewTabButton {
    const NAME: &'static str = "NewTabButton";
}

impl ObjectImpl for NewTabButton {
    fn construct(&mut self) {
        self.parent_construct();

        let dropdown_list = SessionDropdownList::new_alloc();
        connect!(
            dropdown_list,
            visibility_changed(),
            self,
            dropdown_list_visibility_changed(bool)
        );
        self.add_popup(dropdown_list.to_dyn_popup_tr());
    }

    fn initialize(&mut self) {
        EventBus::register(self);

        self.set_valign(Align::Center);
        self.set_margin_left(5);
        self.set_border_radius(6.);
        self.enable_bubble(EventBubble::MOUSE_MOVE);

        // Handle add_tab icon:
        self.add_tab.set_borders(0., 1., 0., 0.);
        self.add_tab.set_border_color(TermdotConfig::separator());
        self.add_tab.width_request(30);
        self.add_tab.height_request(20);
        self.add_tab.set_mouse_tracking(true);

        let showed = self.dropdown_list_showed.clone();
        self.add_tab.register_mouse_enter(move |w| {
            if showed.get() {
                return;
            }
            w.set_background(TermdotConfig::hover());
        });

        let showed = self.dropdown_list_showed.clone();
        self.add_tab.register_mouse_leave(move |w| {
            if showed.get() {
                return;
            }
            w.set_background(Color::TRANSPARENT)
        });

        // Handle drop_down icon:
        self.drop_down.width_request(30);
        self.drop_down.height_request(20);
        self.drop_down.set_mouse_tracking(true);

        let showed = self.dropdown_list_showed.clone();
        self.drop_down.register_mouse_enter(move |w| {
            if showed.get() {
                return;
            }
            w.set_background(TermdotConfig::hover());
        });

        let showed = self.dropdown_list_showed.clone();
        self.drop_down.register_mouse_leave(move |w| {
            if showed.get() {
                return;
            }
            w.set_background(Color::TRANSPARENT)
        });

        connect!(
            self.add_tab,
            mouse_released(),
            self,
            add_tab_released(MouseEvent)
        );
        connect!(
            self.drop_down,
            mouse_pressed(),
            self,
            drop_down_pressed(MouseEvent)
        );
        connect!(
            self.drop_down,
            mouse_released(),
            self,
            drop_down_released(MouseEvent)
        );
    }

    fn on_drop(&mut self) {
        EventBus::remove(self);
    }
}

impl WidgetImpl for NewTabButton {
    #[inline]
    fn on_mouse_move(&mut self, event: &MouseEvent) {
        self.mouse_in_dropdown_icon = self
            .drop_down
            .rect()
            .contains(&self.map_to_global(&event.position().into()));

        self.mouse_in_addtab_icon = self
            .add_tab
            .rect()
            .contains(&self.map_to_global(&event.position().into()));
    }

    #[inline]
    fn on_mouse_enter(&mut self, _: &MouseEvent) {
        if self.dropdown_list_showed.get() {
            return;
        }

        self.set_background(TermdotConfig::pre_hover());
    }

    #[inline]
    fn on_mouse_leave(&mut self, _: &MouseEvent) {
        self.mouse_in_dropdown_icon = false;

        if self.dropdown_list_showed.get() {
            return;
        }

        self.set_background(TermdotConfig::background());
    }
}

impl NewTabButton {
    fn add_tab_released(&mut self, e: MouseEvent) {
        if e.mouse_button() != MouseButton::LeftButton {
            return;
        }

        if self
            .add_tab
            .rect()
            .contains(&self.add_tab.map_to_global(&e.position().into()))
            && self.just_hide
        {
            self.set_background(TermdotConfig::pre_hover());
            self.add_tab.set_background(TermdotConfig::hover());
            self.drop_down.set_background(Color::TRANSPARENT);
        }
    }

    fn drop_down_pressed(&mut self, e: MouseEvent) {
        if e.mouse_button() != MouseButton::LeftButton {
            return;
        }

        self.just_hide = false;
    }

    fn drop_down_released(&mut self, e: MouseEvent) {
        if e.mouse_button() != MouseButton::LeftButton {
            return;
        }
        if self.just_hide {
            if self
                .drop_down
                .rect()
                .contains(&self.drop_down.map_to_global(&e.position().into()))
            {
                self.set_background(TermdotConfig::pre_hover());
                self.drop_down.set_background(TermdotConfig::hover());
                self.add_tab.set_background(Color::TRANSPARENT);
            }
            return;
        }

        if self
            .drop_down
            .rect()
            .contains(&self.drop_down.map_to_global(&e.position().into()))
        {
            self.set_background(TermdotConfig::background());
            self.add_tab.set_background(Color::TRANSPARENT);
            self.drop_down.set_background(TermdotConfig::highlight());

            self.show_popup(self.rect().bottom_left());

            self.dropdown_list_showed.set(true);
        }
    }

    fn dropdown_list_visibility_changed(&mut self, visible: bool) {
        if !visible {
            if !self.mouse_in_dropdown_icon && !self.mouse_in_addtab_icon {
                self.drop_down.set_background(Color::TRANSPARENT);
                self.set_background(TermdotConfig::background());
            }
            self.dropdown_list_showed.set(false);
            self.just_hide = true;
            EventBus::push(Events::SessionDropdownListHide);
        }
    }
}

impl EventHandle for NewTabButton {
    type EventType = EventType;
    type Event = Events;

    #[inline]
    fn listen(&self) -> Vec<Self::EventType> {
        vec![EventType::ThemeChanged]
    }

    #[allow(clippy::single_match)]
    fn handle(&mut self, evt: &Self::Event) {
        match evt {
            Events::ThemeChanged => {
                self.set_background(TermdotConfig::background());
                self.add_tab.set_border_color(TermdotConfig::separator());
                self.add_tab.set_background(TermdotConfig::background());
                self.drop_down.set_background(TermdotConfig::background());
            }

            _ => {}
        }
    }
}
