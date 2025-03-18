use godot::prelude::*;
use ipc::{IPC_DATA_SIZE, ipc_context::IpcContext, ipc_event::IpcEvent};
use widestring::WideString;

use crate::{command::Command, shell::Shell};

/// Main Godot node for plugin status management, and interactive with users.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Termdot {
    #[export]
    host_name: GString,

    ipc_context: Option<IpcContext>,

    shell: Shell,

    base: Base<Node>,
}

#[godot_api]
impl INode for Termdot {
    fn ready(&mut self) {
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

        self.ipc_context = IpcContext::master();
        if self.ipc_context.is_none() {
            godot_warn!("[Termdot::ready] Create master `IpcContext` failed.")
        }

        self.shell.set_prompt(&self.host_name);
    }

    fn process(&mut self, _delta: f64) {
        if self.ipc_context.is_none() {
            return;
        }

        while let Some(evt) = self.ipc_context.as_ref().unwrap().try_recv() {
            match evt {
                IpcEvent::Exit => {}
                IpcEvent::SetTerminalSize(cols, rows) => {
                    self.shell.set_terminal_size(cols, rows);
                }
                IpcEvent::SendData(data, len) => self.recv_data(&data, len),
            }
        }

        let ipc_context = self.ipc_context.as_mut().unwrap();
        while let Some(echo) = self.shell.next_echo() {
            if let Err(e) = ipc_context.try_send(echo) {
                godot_error!("[Termdot::process] Send echo failed, e = {:?}", e);
            }
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
}
