use super::title_bar::TitleBar;
use crate::{
    config::TermdotConfig,
    events::{EventBus, EventType, Events},
    session::Session,
};
use ipc::ipc_context::IpcContext;
use termio::{cli::constant::ProtocolType, emulator::core::terminal_emulator::TerminalEmulator};
use tlib::{event_bus::event_handle::EventHandle, global_watch, iter_executor};
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{IterExecutor, WidgetImpl},
};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
#[iter_executor]
#[global_watch(MouseMove, MousePressed, MouseReleased)]
pub struct App {
    #[children]
    title_bar: Box<TitleBar>,
    #[children]
    terminal_emulator: Box<TerminalEmulator>,

    #[derivative(Default(value = "IpcContext::terminal().unwrap()"))]
    ipc_context: IpcContext,

    resize_zone: bool,
    resize_pressed: bool,
    resize_direction: ResizeDirection,
}

impl ObjectSubclass for App {
    const NAME: &'static str = "App";
}

impl ObjectImpl for App {
    fn initialize(&mut self) {
        EventBus::register(self);

        self.terminal_emulator.set_hexpand(true);
        self.terminal_emulator.set_vexpand(true);

        self.set_vexpand(true);
        self.set_hexpand(true);
    }
}

impl WidgetImpl for App {}

impl GlobalWatchImpl for App {
    #[inline]
    fn on_global_mouse_move(&mut self, evt: &MouseEvent) -> bool {
        let pos: Point = evt.position().into();
        let rect = self.rect();

        let left_top = distance_to(pos, rect.top_left()).abs() <= 6;
        let right_top = distance_to(pos, rect.top_right()).abs() <= 6;
        let right_bottom = distance_to(pos, rect.bottom_right()).abs() <= 6;
        let left_bottom = distance_to(pos, rect.bottom_left()).abs() <= 6;
        let left = pos.x() >= rect.left() - 3 && pos.x() <= rect.left() + 3;
        let right = pos.x() >= rect.right() - 3 && pos.x() <= rect.right() + 3;
        let top = pos.y() >= rect.top() - 3 && pos.y() <= rect.top() + 3;
        let bottom = pos.y() >= rect.bottom() - 3 && pos.y() <= rect.bottom() + 3;

        if left_top || right_top || right_bottom || left_bottom || left || right || top || bottom {
            if !self.resize_pressed {
                if left_top {
                    self.set_cursor_shape(SystemCursorShape::SizeFDiagCursor);
                    self.resize_direction = ResizeDirection::LeftTop;
                } else if right_bottom {
                    self.set_cursor_shape(SystemCursorShape::SizeFDiagCursor);
                    self.resize_direction = ResizeDirection::RightBottom;
                } else if right_top {
                    self.set_cursor_shape(SystemCursorShape::SizeBDiagCursor);
                    self.resize_direction = ResizeDirection::RightTop;
                } else if left_bottom {
                    self.set_cursor_shape(SystemCursorShape::SizeBDiagCursor);
                    self.resize_direction = ResizeDirection::LeftBottom;
                } else if left {
                    self.set_cursor_shape(SystemCursorShape::SizeHorCursor);
                    self.resize_direction = ResizeDirection::Left;
                } else if right {
                    self.set_cursor_shape(SystemCursorShape::SizeHorCursor);
                    self.resize_direction = ResizeDirection::Right;
                } else if top {
                    self.set_cursor_shape(SystemCursorShape::SizeVerCursor);
                    self.resize_direction = ResizeDirection::Top;
                } else if bottom {
                    self.set_cursor_shape(SystemCursorShape::SizeVerCursor);
                    self.resize_direction = ResizeDirection::Bottom;
                }
            }
            self.resize_zone = true;
        } else if self.resize_zone && !self.resize_pressed {
            self.set_cursor_shape(SystemCursorShape::ArrowCursor);
            self.resize_zone = false;
            self.resize_direction = ResizeDirection::None;
        }

        if self.resize_pressed {
            match self.resize_direction {
                ResizeDirection::LeftTop => {
                    let new_width = rect.right() - pos.x();
                    let new_height = rect.bottom() - pos.y();
                    if rect.width() != new_width || rect.height() != new_height {
                        let window = ApplicationWindow::window();

                        let (x_offset, y_offset) =
                            (new_width - rect.width(), new_height - rect.height());
                        let mut outer_position = window.outer_position();
                        outer_position.offset(-x_offset, -y_offset);

                        window.resize(Some(new_width), Some(new_height));
                        window.request_win_position(outer_position);
                    }
                }
                ResizeDirection::RightTop => {
                    let new_width = pos.x() - rect.left();
                    let new_height = rect.bottom() - pos.y();
                    if rect.width() != new_width || rect.height() != new_height {
                        let window = ApplicationWindow::window();

                        let y_offset = new_height - rect.height();
                        let mut outer_position = window.outer_position();
                        outer_position.offset(0, -y_offset);

                        window.resize(Some(new_width), Some(new_height));
                        window.request_win_position(outer_position);
                    }
                }
                ResizeDirection::RightBottom => {
                    let new_width = pos.x() - rect.left();
                    let new_height = pos.y() - rect.top();
                    if rect.width() != new_width || rect.height() != new_height {
                        ApplicationWindow::window().resize(Some(new_width), Some(new_height));
                    }
                }
                ResizeDirection::LeftBottom => {
                    let new_width = rect.right() - pos.x();
                    let new_height = pos.y() - rect.top();
                    if rect.width() != new_width || rect.height() != new_height {
                        let window = ApplicationWindow::window();

                        let x_offset = new_width - rect.width();
                        let mut outer_position = window.outer_position();
                        outer_position.offset(-x_offset, 0);

                        window.resize(Some(new_width), Some(new_height));
                        window.request_win_position(outer_position);
                    }
                }
                ResizeDirection::Left => {
                    let new_width = rect.right() - pos.x();
                    if rect.width() != new_width {
                        let window = ApplicationWindow::window();

                        let x_offset = new_width - rect.width();
                        let mut outer_position = window.outer_position();
                        outer_position.offset(-x_offset, 0);

                        window.resize(Some(new_width), None);
                        window.request_win_position(outer_position);
                    }
                }
                ResizeDirection::Right => {
                    let new_width = pos.x() - rect.left();
                    if rect.width() != new_width {
                        ApplicationWindow::window().resize(Some(new_width), None);
                    }
                }
                ResizeDirection::Top => {
                    let new_height = rect.bottom() - pos.y();
                    if rect.height() != new_height {
                        let window = ApplicationWindow::window();

                        let y_offset = new_height - rect.height();
                        let mut outer_position = window.outer_position();
                        outer_position.offset(0, -y_offset);

                        window.resize(None, Some(new_height));
                        window.request_win_position(outer_position);
                    }
                }
                ResizeDirection::Bottom => {
                    let new_height = pos.y() - rect.top();
                    if rect.height() != new_height {
                        ApplicationWindow::window().resize(None, Some(new_height));
                    }
                }
                _ => return false,
            }
            true
        } else {
            false
        }
    }

    #[inline]
    fn on_global_mouse_pressed(&mut self, _: &MouseEvent) -> bool {
        if self.resize_zone {
            self.resize_pressed = true;
            true
        } else {
            false
        }
    }

    fn on_global_mouse_released(&mut self, evt: &MouseEvent) -> bool {
        let pos: Point = evt.position().into();
        let rect = self.rect();

        if self.resize_pressed {
            self.resize_pressed = false;
        }
        self.resize_direction = ResizeDirection::None;

        let left_top = distance_to(pos, rect.top_left()).abs() <= 6;
        let right_top = distance_to(pos, rect.top_right()).abs() <= 6;
        let right_bottom = distance_to(pos, rect.bottom_right()).abs() <= 6;
        let left_bottom = distance_to(pos, rect.bottom_left()).abs() <= 6;
        let left = pos.x() >= rect.left() - 3 && pos.x() <= rect.left() + 3;
        let right = pos.x() >= rect.right() - 3 && pos.x() <= rect.right() + 3;
        let top = pos.y() >= rect.top() - 3 && pos.y() <= rect.top() + 3;
        let bottom = pos.y() >= rect.bottom() - 3 && pos.y() <= rect.bottom() + 3;

        if left_top || right_top || right_bottom || left_bottom || left || right || top || bottom {
            if left_top {
                self.set_cursor_shape(SystemCursorShape::SizeFDiagCursor);
                self.resize_direction = ResizeDirection::LeftTop;
            } else if right_bottom {
                self.set_cursor_shape(SystemCursorShape::SizeFDiagCursor);
                self.resize_direction = ResizeDirection::RightBottom;
            } else if right_top {
                self.set_cursor_shape(SystemCursorShape::SizeBDiagCursor);
                self.resize_direction = ResizeDirection::RightTop;
            } else if left_bottom {
                self.set_cursor_shape(SystemCursorShape::SizeBDiagCursor);
                self.resize_direction = ResizeDirection::LeftBottom;
            } else if left {
                self.set_cursor_shape(SystemCursorShape::SizeHorCursor);
                self.resize_direction = ResizeDirection::Left;
            } else if right {
                self.set_cursor_shape(SystemCursorShape::SizeHorCursor);
                self.resize_direction = ResizeDirection::Right;
            } else if top {
                self.set_cursor_shape(SystemCursorShape::SizeVerCursor);
                self.resize_direction = ResizeDirection::Top;
            } else if bottom {
                self.set_cursor_shape(SystemCursorShape::SizeVerCursor);
                self.resize_direction = ResizeDirection::Bottom;
            }
            self.resize_zone = true;
        } else if self.resize_zone && !self.resize_pressed {
            self.set_cursor_shape(SystemCursorShape::ArrowCursor);
            self.resize_zone = false;
        }

        false
    }
}

impl EventHandle for App {
    type EventType = EventType;
    type Event = Events;

    #[inline]
    fn listen(&self) -> Vec<Self::EventType> {
        vec![
            EventType::HeartBeatUndetected,
            EventType::ThemeChanged,
            EventType::FontChanged,
        ]
    }

    #[inline]
    fn handle(&mut self, evt: &Self::Event) {
        match evt {
            Events::HeartBeatUndetected => {
                // Do nothing temporary
            }
            Events::ThemeChanged => {
                self.on_theme_changed();
            }
            Events::FontChanged => {
                self.terminal_emulator
                    .set_terminal_font(TermdotConfig::font());
            }
            _ => {}
        }
    }
}

#[inline]
fn distance_to(from: Point, to: Point) -> i32 {
    (((to.x() - from.x()).pow(2) + (to.y() - from.y()).pow(2)) as f64).sqrt() as i32
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum ResizeDirection {
    #[default]
    None,
    LeftTop,
    RightTop,
    RightBottom,
    LeftBottom,
    Left,
    Right,
    Top,
    Bottom,
}

impl IterExecutor for App {
    #[inline]
    fn iter_execute(&mut self) {
        while let Some(session_id) = self.ipc_context.try_recv() {
            EventBus::push(Events::CreateSession(Session::new(
                session_id,
                ProtocolType::Custom,
            )));
        }

        EventBus::process();
    }
}

impl App {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn on_theme_changed(&mut self) {
        let background = TermdotConfig::background();
        self.window().set_background(background);
        self.set_background(background);
        self.terminal_emulator.set_background(background);

        self.terminal_emulator
            .set_theme(&TermdotConfig::get_theme());
    }
}
