use crate::{
    command::execute_status::ShExecuteStatus,
    termdot::{shell_version, terminal_version},
    utils::ansi_string::rust::ShAnsiString,
};

use super::IInternalCommand;
use godot::prelude::*;

pub struct CmdVersion;

impl IInternalCommand for CmdVersion {
    #[inline]
    fn command_name(&self) -> String {
        "version".to_string()
    }

    #[inline]
    fn start(&mut self, _params: Array<GString>) -> ShExecuteStatus {
        let shell_version = shell_version();
        let terminal_version = terminal_version();
        let ansi_str = ShAnsiString::new()
            .append(&format!("Termdot Shell Version: {}\r\n", shell_version))
            .append(&format!(
                "Termdot Terminal Version: {}\r\n",
                terminal_version
            ));
        self.echo(ansi_str);
        ShExecuteStatus::Done
    }
}
