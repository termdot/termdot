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
    /// The register command name.
    /// If it is the same as an internal command, it will be ignored.
    /// If it is the same as the previous command, the previous one will be overwritten.
    command_name: GString,
    base: Base<Node>,
}

#[allow(unused_variables)]
#[godot_api]
impl Command {
    #[func(virtual, gd_self)]
    /// This method is executed when the command is detected.
    /// The command is trimmed by spaces, and parameters are passed as `params`.
    /// Return value: ExecuteStatus.DONE or ExecuteStatus.RUNNING.
    pub fn start(gd: Gd<Self>, params: Array<GString>) -> ShExecuteStatus {
        ShExecuteStatus::Done
    }

    #[func(virtual, gd_self)]
    /// This method executes when `_start()` returns `ExecuteStatus.RUNNING` and continues
    /// running until `_running()` itself returns `ExecuteStatus.DONE`.
    /// Return value: ExecuteStatus.DONE or ExecuteStatus.RUNNING.
    pub fn running(gd: Gd<Self>) -> ShExecuteStatus {
        ShExecuteStatus::Done
    }

    #[func(virtual, gd_self)]
    /// Executed when receive interrupt signal(Control+C)
    /// Do nothing by default.
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
    /// Send echo to terminal.
    pub fn echo(&self, text: Gd<AnsiString>) {
        SHELL.with(|rf| nonnull_mut!(rf.borrow_mut()).echo(text))
    }
}
