use std::collections::VecDeque;

use super::IInternalCommand;
use crate::{
    command::execute_status::ShExecuteStatus,
    utils::{ansi_string::rust::ShAnsiString, color256::Color256},
};
use derivative::Derivative;
use godot::builtin::{Array, GString};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use tmui::tlib::utils::Timestamp;

lazy_static! {
    static ref LOGS: Mutex<VecDeque<ShAnsiString>> = Mutex::new(VecDeque::default());
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct CmdLog {
    #[derivative(Default(value = "500"))]
    max_peek: usize,
}

impl IInternalCommand for CmdLog {
    #[inline]
    fn command_name(&self) -> String {
        "log".to_string()
    }

    #[inline]
    fn start(&mut self, _params: Array<GString>) -> ShExecuteStatus {
        ShExecuteStatus::Running
    }

    #[inline]
    fn running(&mut self) -> ShExecuteStatus {
        let mut lock = LOGS.lock();
        let mut echo = ShAnsiString::new();

        let mut peeked = 0;
        while let Some(log) = lock.pop_front() {
            echo = echo.append(log.append("\r\n").as_str());
            peeked += 1;

            if peeked >= self.max_peek {
                break;
            }
        }

        self.echo(echo);
        ShExecuteStatus::Running
    }
}

impl CmdLog {
    pub fn info(log: String) {
        let time = format!("[{}] ", Timestamp::now().format_string(None));

        let ansi_log = ShAnsiString::new()
            .append(&time)
            .append("[")
            .foreground_256(Color256::GREEN)
            .append("INFO")
            .clear_style()
            .append("] ")
            .append(&log);

        LOGS.lock().push_back(ansi_log);
    }

    pub fn warn(log: String) {
        let time = format!("[{}] ", Timestamp::now().format_string(None));

        let ansi_log = ShAnsiString::new()
            .append(&time)
            .append("[")
            .foreground_256(Color256::YELLOW)
            .append("WARN")
            .clear_style()
            .append("] ")
            .append(&log);

        LOGS.lock().push_back(ansi_log);
    }

    pub fn error(log: String) {
        let time = format!("[{}] ", Timestamp::now().format_string(None));

        let ansi_log = ShAnsiString::new()
            .append(&time)
            .append("[")
            .foreground_256(Color256::RED)
            .append("ERROR")
            .clear_style()
            .append("] ")
            .append(&log);

        LOGS.lock().push_back(ansi_log);
    }
}
