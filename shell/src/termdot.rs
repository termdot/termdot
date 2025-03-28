use crate::{command::Command, shell::Shell};
use godot::{
    classes::{InputEvent, InputMap, ProjectSettings, notify::NodeNotification},
    prelude::*,
};
use ipc::{
    IPC_DATA_SIZE,
    ipc_context::{IpcContext, SHARED_ID},
    ipc_event::IpcEvent,
};
use std::{process::Child, str::FromStr};
use tmui::tlib::utils::SnowflakeGuidGenerator;
use widestring::WideString;

#[cfg(target_os = "windows")]
pub const APP_PATH: &str = "res://addons/termdot/termdot.exe";

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

    ipc_context: Option<IpcContext>,

    shell: Shell,
    child: Option<Child>,

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

    fn process(&mut self, _delta: f64) {
        self.shell.process_running_command();

        if self.ipc_context.is_none() {
            return;
        }

        while let Some(evt) = self.ipc_context.as_ref().unwrap().try_recv() {
            match evt {
                IpcEvent::Ready => self.shell.prompt(),
                IpcEvent::Exit => {
                    self.child = None;
                    self.shell.reset();
                }
                IpcEvent::SetTerminalSize(cols, rows) => {
                    self.shell.set_terminal_size(cols, rows);
                }
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
        {
            if self.child.is_none() {
                self.start_sub_process();
            }
        }
    }

    fn exit_tree(&mut self) {
        self.termdot_exit();
    }

    #[allow(clippy::single_match)]
    fn on_notification(&mut self, what: NodeNotification) {
        match what {
            NodeNotification::EXIT_TREE
            | NodeNotification::UNPARENTED
            | NodeNotification::WM_CLOSE_REQUEST
            | NodeNotification::CRASH => {
                self.termdot_exit();
            }
            _ => {}
        }
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

        let wstr = WideString::from_str(&data);
        for &c in wstr.as_slice() {
            self.shell.receive_char(c);
        }
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

    fn start_sub_process(&mut self) {
        let id = SHARED_ID.load(std::sync::atomic::Ordering::Relaxed);
        let path = ProjectSettings::singleton()
            .globalize_path(APP_PATH)
            .to_string();
        match std::process::Command::new(path).arg(id.to_string()).spawn() {
            Ok(c) => self.child = Some(c),
            Err(e) => godot_error!("Run external app failed, e = {:?}", e),
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
}
