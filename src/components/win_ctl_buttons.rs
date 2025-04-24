use crate::{
    assets::Asset,
    components::title_bar::TITLE_BAR_HEIGHT,
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
};
use tlib::event_bus::event_handle::EventHandle;
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
        self.width_request(135);

        let background = self.background();

        self.minimize.width_request(45);
        self.minimize.height_request(TITLE_BAR_HEIGHT - 1);
        self.minimize
            .register_mouse_enter(|w| w.set_background(TermdotConfig::hover()));
        self.minimize
            .register_mouse_leave(move |w| w.set_background(background));
        self.minimize
            .register_mouse_released(|w, _| w.window().minimize());

        self.maximize_restore.width_request(45);
        self.maximize_restore.height_request(TITLE_BAR_HEIGHT - 1);
        self.maximize_restore
            .register_mouse_enter(|w| w.set_background(TermdotConfig::hover()));
        self.maximize_restore
            .register_mouse_leave(move |w| w.set_background(background));
        self.maximize_restore.register_mouse_released(|w, _| {
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
        self.close
            .register_mouse_released(|w, _| w.window().close());
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
    fn handle(&mut self, evt: &Self::Event) {
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
