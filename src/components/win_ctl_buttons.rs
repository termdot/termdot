use crate::{
    assets::Asset,
    components::title_bar::TITLE_BAR_HEIGHT,
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
};
use tlib::{event_bus::event_handle::EventHandle, winit::window::WindowLevel};
use tmui::{
    icons::{svg_icon::SvgIcon, svg_toggle_icon::SvgToggleIcon},
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{callbacks::CallbacksRegister, WidgetImpl},
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct WinControlButtons {
    #[derivative(Default(value = "{
        let file = Asset::get(\"icons/pinned.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    pinned: Tr<SvgIcon>,

    #[derivative(Default(value = "{
        let file = Asset::get(\"icons/minimize.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    minimize: Tr<SvgIcon>,

    #[derivative(Default(value = "{
        let maximize = Asset::get(\"icons/large.svg\").unwrap();
        let restore = Asset::get(\"icons/restore.svg\").unwrap();
        SvgToggleIcon::from_bytes(&[maximize.data.as_ref(), restore.data.as_ref()])
    }"))]
    #[children]
    maximize_restore: Tr<SvgToggleIcon>,

    #[derivative(Default(value = "{
        let file = Asset::get(\"icons/close.svg\").unwrap();
        SvgIcon::from_bytes(file.data.as_ref())
    }"))]
    #[children]
    close: Tr<SvgIcon>,
}

impl ObjectSubclass for WinControlButtons {
    const NAME: &'static str = "WinControlButtons";
}

impl ObjectImpl for WinControlButtons {
    fn initialize(&mut self) {
        EventBus::register(self);

        self.set_halign(Align::End);
        self.set_vexpand(true);
        let width = 45 * 4;
        self.width_request(width);
        self.set_size_hint(SizeHint::new().with_max_width(width).with_min_width(width));

        let background = self.background();

        self.pinned.width_request(45);
        self.pinned.height_request(TITLE_BAR_HEIGHT - 1);
        self.pinned
            .register_mouse_enter(|w| w.set_background(TermdotConfig::hover()));
        self.pinned.register_mouse_leave(move |w| {
            let pinned = w
                .get_property("pinned")
                .map(|val| val.get::<bool>())
                .unwrap_or_default();
            if !pinned {
                w.set_background(background)
            }
        });
        self.pinned.register_mouse_released(|w, event| {
            let pos = w.map_to_global(&event.position().into());
            if !w.rect().contains(&pos) {
                return;
            }

            let pinned = w
                .get_property("pinned")
                .map(|val| val.get::<bool>())
                .unwrap_or_default();

            if pinned {
                ApplicationWindow::window().set_window_level(WindowLevel::Normal);
            } else {
                ApplicationWindow::window().set_window_level(WindowLevel::AlwaysOnTop);
            }

            w.set_property("pinned", (!pinned).to_value());
        });

        self.minimize.width_request(45);
        self.minimize.height_request(TITLE_BAR_HEIGHT - 1);
        self.minimize
            .register_mouse_enter(|w| w.set_background(TermdotConfig::hover()));
        self.minimize
            .register_mouse_leave(move |w| w.set_background(background));
        self.minimize.register_mouse_released(|w, event| {
            let pos = w.map_to_global(&event.position().into());
            if !w.rect().contains(&pos) {
                return;
            }
            w.window().minimize()
        });

        self.maximize_restore.width_request(45);
        self.maximize_restore.height_request(TITLE_BAR_HEIGHT - 1);
        self.maximize_restore
            .register_mouse_enter(|w| w.set_background(TermdotConfig::hover()));
        self.maximize_restore
            .register_mouse_leave(move |w| w.set_background(background));
        self.maximize_restore.register_mouse_released(|w, event| {
            let pos = w.map_to_global(&event.position().into());
            if !w.rect().contains(&pos) {
                return;
            }

            let icon = w.downcast_mut::<SvgToggleIcon>().unwrap();
            match icon.current_icon() {
                0 => icon.window().maximize(),
                1 => icon.window().restore(),
                _ => unreachable!(),
            }
        });
        self.maximize_restore.register_window_maximized(|w| {
            w.downcast_mut::<SvgToggleIcon>()
                .unwrap()
                .set_current_icon(1)
        });
        self.maximize_restore.register_window_restored(|w| {
            w.downcast_mut::<SvgToggleIcon>()
                .unwrap()
                .set_current_icon(0)
        });

        self.close.width_request(45);
        self.close.height_request(TITLE_BAR_HEIGHT - 1);
        self.close
            .register_mouse_enter(|w| w.set_background(TermdotConfig::error()));
        self.close
            .register_mouse_leave(move |w| w.set_background(background));
        self.close.register_mouse_released(|w, event| {
            let pos = w.map_to_global(&event.position().into());
            if !w.rect().contains(&pos) {
                return;
            }
            w.window().close();
        });
    }

    fn on_drop(&mut self) {
        EventBus::remove(self);
    }
}

impl WidgetImpl for WinControlButtons {}

impl EventHandle for WinControlButtons {
    type EventType = EventType;
    type Event = Events;

    #[inline]
    fn listen(&self) -> Vec<Self::EventType> {
        vec![EventType::ThemeChanged]
    }

    #[allow(clippy::single_match)]
    #[inline]
    fn handle_evt(&mut self, evt: &Self::Event) {
        match evt {
            Events::ThemeChanged => {
                let background = TermdotConfig::background();
                self.set_background(background);

                self.minimize
                    .register_mouse_enter(|w| w.set_background(TermdotConfig::hover()));
                self.minimize
                    .register_mouse_leave(move |w| w.set_background(background));

                self.maximize_restore
                    .register_mouse_enter(|w| w.set_background(TermdotConfig::hover()));
                self.maximize_restore
                    .register_mouse_leave(move |w| w.set_background(background));

                self.close
                    .register_mouse_enter(|w| w.set_background(TermdotConfig::error()));
                self.close
                    .register_mouse_leave(move |w| w.set_background(background));
            }

            _ => {}
        }
    }
}
