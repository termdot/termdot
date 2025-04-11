pub mod command;
pub mod consoel_captures;
pub mod plugin;
pub mod shell;
pub mod termdot;
pub mod utils;

use common::log::LocalLog;
use godot::prelude::*;
use tmui::prelude::ActionHub;

struct TermdotShell;

#[gdextension(entry_symbol = termdot_init)]
unsafe impl ExtensionLibrary for TermdotShell {
    #[allow(clippy::single_match)]
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Core => {
                std::panic::set_hook(Box::new(|panic_info| {
                    let thread = std::thread::current();
                    let thread_name = thread.name().unwrap_or("unnamed");
                    let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                        *s
                    } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                        s.as_str()
                    } else {
                        "Unknown panic"
                    };
                    let location = panic_info.location();
                    LocalLog::append(format!(
                        "[PANIC] [{}] Panic occurred.\r\nLocation: {:?}\r\nPanic:\r\n{}\r\n",
                        thread_name, location, msg
                    ));
                }));

                ActionHub::initialize();
            }
            _ => {}
        }
    }
}
