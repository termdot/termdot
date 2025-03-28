pub mod cls;

use crate::{shell::SHELL, utils::ansi_string::rust::ShAnsiString};
use godot::builtin::{Array, GString, Vector2i};
use tmui::tlib::nonnull_mut;

use super::execute_status::ShExecuteStatus;

pub type InternalCommand = Box<dyn IInternalCommand>;
pub type InternalCommandHnd = *mut dyn IInternalCommand;

pub trait IInternalCommand {
    fn command_name(&self) -> String;

    fn start(&mut self, params: Array<GString>) -> ShExecuteStatus;

    #[inline]
    fn running(&mut self) -> ShExecuteStatus {
        ShExecuteStatus::Done
    }

    #[inline]
    /// Get current terminal size, represent as (cols, rows)
    fn get_terminal_size(&self) -> Vector2i {
        SHELL.with(|rf| nonnull_mut!(rf.borrow_mut()).get_terminal_size())
    }

    #[inline]
    /// Get current cursor position, represent as (cols, rows)
    ///
    /// The origin point of cursor is (1, 1)
    fn get_cursor_position(&self) -> Vector2i {
        SHELL.with(|rf| nonnull_mut!(rf.borrow_mut()).get_cursor_position())
    }

    #[inline]
    fn echo(&self, text: ShAnsiString) {
        SHELL.with(|rf| nonnull_mut!(rf.borrow_mut()).sh_echo(text))
    }
}
