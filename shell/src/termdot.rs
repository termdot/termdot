use crate::{
    command::{Command, internal::log::CmdLog},
    shell::Shell,
};
use godot::{
    classes::{InputEvent, InputMap, ProjectSettings, notify::NodeNotification},
    prelude::*,
};
use ipc::{
    HEART_BEAT_INTERVAL, IPC_DATA_SIZE,
    ipc_context::{IpcContext, SHARED_ID},
    ipc_event::IpcEvent,
};
use std::{cell::RefCell, process::Child, str::FromStr, time::Instant};
use tmui::tlib::{global::SemanticExt, utils::SnowflakeGuidGenerator};
use wchar::wchar_t;
use widestring::WideString;

const VERSION: &str = env!("CARGO_PKG_VERSION");
thread_local! {
    static TERMINAL_VERSION: RefCell<String> = const { RefCell::new(String::new()) };
}

#[inline]
pub fn shell_version() -> &'static str {
    VERSION
}
#[inline]
pub fn terminal_version() -> &'static str {
    TERMINAL_VERSION.with(|br| Box::leak(br.borrow().clone().boxed()))
}

#[cfg(windows_platform)]
pub const APP_PATH: [&str; 2] = ["res://addons/termdot/termdot.exe", "res://termdot.exe"];
#[cfg(macos_platform)]
pub const APP_PATH: [&str; 1] = [""];
#[cfg(free_unix)]
pub const APP_PATH: [&str; 1] = [""];

#[derive(GodotClass)]
/// Main Godot node for plugin status management, and interactive with users.
#[class(init, base = Node)]
#[allow(dead_code)]
pub struct Termdot {
    #[export]
    /// Host name of shell, will represent as `host_name> `.
    #[init(val = GString::from_str("termdot").unwrap())]
    host_name: GString,

    #[export]
    /// External terminal will run automatically when ready.
    #[init(val = true)]
    auto_run: bool,

    #[export]
    /// When action `run_action` detected pressed, run the external terminal if it's not running.
    #[init(val = GString::from_str("termdot_run").unwrap())]
    run_action: GString,

    #[export(range = (1., 60.))]
    /// Commands execution frequency
    #[init(val = 60)]
    command_ticks_per_second: u32,
    accumulator: f64,

    ipc_context: Option<IpcContext>,

    shell: Shell,
    child: Option<Child>,
    #[init(val = Instant::now())]
    last_heart_beat: Instant,

    base: Base<Node>,
}

#[godot_api]
impl INode for Termdot {
    fn ready(&mut self) {
        self.shell.init();

        self.shell.init_internal_command();

        for child in self.base().get_children().iter_shared() {
            if let Ok(command) = child.try_cast::<Command>() {
                let name = command.bind().get_command_name().to_string();
                if name.is_empty() {
                    godot_warn!(
                        "[Termdot::ready] The `command_name` of Command {} can't be empty, ignore command register.",
                        command.get_name()
                    );
                    continue;
                }
                self.shell.insert_command(name, command);
            }
        }

        let id = SnowflakeGuidGenerator::next_id().unwrap();
        SHARED_ID.store(id, std::sync::atomic::Ordering::Release);

        self.ipc_context = IpcContext::master();
        if self.ipc_context.is_none() {
            godot_warn!("[Termdot::ready] Create master `IpcContext` failed.")
        }

        self.shell.set_prompt(&self.host_name);

        if self.auto_run {
            self.start_sub_process();
        }
    }

    fn process(&mut self, delta: f64) {
        self.accumulator += delta;
        if self.accumulator >= 1. / self.command_ticks_per_second as f64 {
            self.accumulator = 0.;
            self.shell.process_running_command();
        }

        if self.ipc_context.is_none() {
            return;
        }

        self.heart_beat();

        self.shell.check_buffer_storage();

        while let Some(evt) = self.ipc_context.as_ref().unwrap().try_recv() {
            match evt {
                IpcEvent::HeartBeat => {}
                IpcEvent::Ready => self.shell.prompt(),
                IpcEvent::Exit => {
                    self.child = None;
                    self.shell.reset();
                    self.shell.interrupt(false);
                }
                IpcEvent::TerminalVersion(data, len) => self.set_terminal_version(data, len),
                IpcEvent::SetTerminalSize(cols, rows) => self.shell.set_terminal_size(cols, rows),
                IpcEvent::SendData(data, len) => self.recv_data(&data, len),
                _ => {}
            }
        }

        let ipc_context = self.ipc_context.as_ref().unwrap();
        while let Some(echo) = self.shell.next_echo() {
            if let Err(e) = ipc_context.try_send(echo) {
                godot_error!("[Termdot::process] Send echo failed, e = {:?}", e);
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let input_map = InputMap::singleton();
        if input_map.has_action(&self.run_action.to_string())
            && event.is_action_pressed(&self.run_action.to_string())
            && self.child.is_none()
        {
            self.start_sub_process();
        }
    }

    fn exit_tree(&mut self) {
        godot_print!("Exit tree");
        self.termdot_exit();
    }

    #[allow(clippy::single_match)]
    fn on_notification(&mut self, what: NodeNotification) {
        match what {
            NodeNotification::EXIT_TREE
            | NodeNotification::UNPARENTED
            | NodeNotification::WM_CLOSE_REQUEST
            | NodeNotification::CRASH => {
                godot_print!("Window closed");
                self.termdot_exit();
            }
            _ => {}
        }
    }
}

#[godot_api]
impl Termdot {
    #[func]
    /// log level info, display by internal command `log`
    pub fn info(log: GString) {
        CmdLog::info(log.to_string());
    }

    #[func]
    /// log level warn, display by internal command `log`
    pub fn warn(log: GString) {
        CmdLog::warn(log.to_string());
    }

    #[func]
    /// log level error, display by internal command `log`
    pub fn error(log: GString) {
        CmdLog::error(log.to_string());
    }
}

impl Termdot {
    fn recv_data(&mut self, data: &[u8; IPC_DATA_SIZE], len: usize) {
        let mut data = data.to_vec();
        data.truncate(len);
        let data = match String::from_utf8(data) {
            Ok(d) => d,
            Err(e) => {
                godot_error!(
                    "[Termdot::recv_data] Parse utf-8 string failed, err = {:?}",
                    e
                );
                return;
            }
        };

        if len > 1 {
            self.shell.set_replay_hint(true);
        }

        let wstr = WideString::from_str(&data);
        for &c in wstr.as_slice() {
            #[allow(clippy::useless_transmute)]
            let c: wchar_t = unsafe { std::mem::transmute(c) };
            self.shell.receive_char(c);
        }

        self.shell.echo_replay_text();
    }

    fn set_terminal_version(&self, data: [u8; IPC_DATA_SIZE], len: usize) {
        let mut data = data.to_vec();
        data.truncate(len);
        let version = match String::from_utf8(data) {
            Ok(v) => v,
            Err(e) => {
                godot_error!(
                    "[Termdot::recv_data] Parse utf-8 string failed, err = {:?}",
                    e
                );
                "UNKNOWN_VERSION".to_string()
            }
        };
        TERMINAL_VERSION.with(|rf| *rf.borrow_mut() = version);
    }

    fn termdot_exit(&mut self) {
        godot_print!("Termdot exit.");

        if let Some(ctx) = self.ipc_context.as_ref() {
            let _ = ctx.try_send(IpcEvent::Exit);
        }
        self.ipc_context = None;

        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
        }
    }

    #[allow(unreachable_code)]
    fn start_sub_process(&mut self) {
        #[cfg(macos_platform)]
        {
            godot_warn!("Termdot is currently not supported on macOS.");
            return;
        }
        #[cfg(free_unix)]
        {
            godot_warn!("Termdot is currently not supported on Linux.");
            return;
        }
        let id = SHARED_ID.load(std::sync::atomic::Ordering::Relaxed);
        for app_path in APP_PATH {
            let path = ProjectSettings::singleton()
                .globalize_path(app_path)
                .to_string();
            match std::process::Command::new(path).arg(id.to_string()).spawn() {
                Ok(c) => {
                    self.child = Some(c);
                    break;
                }
                Err(_) => {}
            }
        }

        if self.child.is_none() {
            godot_error!("Run external app failed, cant find the external application.");
            return;
        }

        self.send_ipc_event(IpcEvent::Ready);
        self.send_ipc_event(IpcEvent::pack_host_name(&self.host_name.to_string()));
    }

    fn send_ipc_event(&self, event: IpcEvent) {
        if let Some(ipc_ctx) = self.ipc_context.as_ref() {
            if let Err(e) = ipc_ctx.try_send(event) {
                godot_error!("[Termdot::process] Send ipc event failed, e = {:?}", e);
            }
        }
    }

    fn heart_beat(&mut self) {
        if self.child.is_none() {
            return;
        }

        if self.last_heart_beat.elapsed().as_millis() >= HEART_BEAT_INTERVAL {
            self.last_heart_beat = Instant::now();
            self.send_ipc_event(IpcEvent::HeartBeat);
        }
    }
}
