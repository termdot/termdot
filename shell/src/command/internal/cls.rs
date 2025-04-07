use super::IInternalCommand;
use crate::{command::execute_status::ShExecuteStatus, utils::ansi_string::rust::ShAnsiString};
use godot::builtin::{Array, GString};

pub struct CmdCls;

impl IInternalCommand for CmdCls {
    #[inline]
    fn command_name(&self) -> String {
        "cls".to_string()
    }

    fn start(&mut self, _: Array<GString>) -> ShExecuteStatus {
        let echo = ShAnsiString::default()
            .clear_entire_screen()
            .cursor_move_to(1, 1);
        self.echo(echo);

        ShExecuteStatus::Done
    }
}
