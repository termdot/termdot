use std::str::FromStr;

use ahash::AHashMap;
use godot::prelude::*;
use ipc::{IPC_DATA_SIZE, ipc_context::IpcContext, ipc_event::IpcEvent};

use crate::command::Command;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Termdot {
    #[export]
    host_name: GString,

    ipc_context: Option<IpcContext>,

    command_map: AHashMap<String, Gd<Command>>,

    terminal_size: Vector2i,

    base: Base<Node>,
}

#[godot_api]
impl INode for Termdot {
    fn ready(&mut self) {
        for child in self.base().get_children().iter_shared() {
            if let Ok(command) = child.try_cast::<Command>() {
                let name = command.bind().get_command_name().to_string();
                self.command_map.insert(name, command);
            }
        }

        self.ipc_context = IpcContext::master();
        if self.ipc_context.is_none() {
            godot_warn!("[Termdot::ready] Create master `IpcContext` failed.")
        }
    }

    fn process(&mut self, _delta: f64) {
        let ipc_context = match self.ipc_context.as_ref() {
            Some(ctx) => ctx,
            None => return,
        };

        while let Some(evt) = ipc_context.try_recv() {
            match evt {
                IpcEvent::Exit => {}
                IpcEvent::SetTerminalSize(cols, rows) => {
                    self.terminal_size = Vector2i::new(cols, rows)
                }
                IpcEvent::SendData(data, len) => {
                    if let Some(chunks) = self.recv_data(&data, len) {
                        for chunk in chunks {
                            if let Err(e) = ipc_context.try_send(chunk) {
                                godot_error!(
                                    "[Termdot::process] Send ipc data failed, e = {:?}",
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Termdot {
    fn recv_data(&self, data: &[u8; IPC_DATA_SIZE], len: usize) -> Option<Vec<IpcEvent>> {
        let mut data = data.to_vec();
        data.truncate(len);
        let data = match String::from_utf8(data) {
            Ok(d) => d,
            Err(e) => {
                godot_error!(
                    "[Termdot::process] Parse utf-8 string failed, err = {:?}",
                    e
                );
                return None;
            }
        };

        let commands = data.trim().split(" ");
        let (mut command, mut params) = (None, array![]);
        for (i, c) in commands.into_iter().enumerate() {
            if i == 0 {
                command = Some(c);
            } else {
                params.push(&GString::from_str(c).unwrap());
            }
        }

        let command = match command {
            Some(c) => c,
            None => return None,
        };

        if let Some(gd) = self.command_map.get(command) {
            Command::start(gd.clone(), params);
            return None;
        } else {
            let send_back = format!(
                "\r\n{}> \u{200B}`{}` is not recognized as an internal command.",
                self.host_name, command,
            );

            return Some(IpcEvent::pack_data(send_back));
        }
    }
}
