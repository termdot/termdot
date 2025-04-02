pub mod execute_status;
pub mod internal;

use crate::{shell::SHELL, utils::ansi_string::godot::AnsiString};
use execute_status::ShExecuteStatus;
use godot::prelude::*;
use tmui::tlib::nonnull_mut;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Command {
    #[export]
    command_name: GString,
    base: Base<Node>,
}

#[allow(unused_variables)]
#[godot_api]
impl Command {
    #[func(virtual, gd_self)]
    pub fn start(gd: Gd<Self>, params: Array<GString>) -> ShExecuteStatus {
        ShExecuteStatus::Done
    }

    #[func(virtual, gd_self)]
    pub fn running(gd: Gd<Self>) -> ShExecuteStatus {
        ShExecuteStatus::Done
    }

    #[func(virtual, gd_self)]
    pub fn interrupting(gd: Gd<Self>) {}

    #[func]
    /// Get current terminal size, represent as (cols, rows)
    pub fn get_terminal_size(&self) -> Vector2i {
        SHELL.with(|rf| nonnull_mut!(rf.borrow_mut()).get_terminal_size())
    }

    #[func]
    /// Get current cursor position, represent as (cols, rows)
    ///
    /// The origin point of cursor is (1, 1)
    pub fn get_cursor_position(&self) -> Vector2i {
        SHELL.with(|rf| nonnull_mut!(rf.borrow_mut()).get_cursor_position())
    }

    #[func]
    pub fn echo(&self, text: Gd<AnsiString>) {
        SHELL.with(|rf| nonnull_mut!(rf.borrow_mut()).echo(text))
    }
}
