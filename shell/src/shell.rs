use crate::command::Command;
use crate::utils::charmap::*;
use ahash::AHashMap;
use derivative::Derivative;
use godot::{
    builtin::{GString, Vector2i, array},
    obj::Gd,
};
use ipc::ipc_event::IpcEvent;
use std::{collections::VecDeque, str::FromStr};
use termio::emulator::emulation::{Emulation, VT102Emulation};
use wchar::wchar_t;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Shell {
    prompt: String,
    command: Vec<wchar_t>,
    cursor: usize,
    is_executing: bool,
    u_stack: Vec<Vec<wchar_t>>,
    d_stack: Vec<Vec<wchar_t>>,
    argv: [wchar_t; 2],

    command_map: AHashMap<String, Gd<Command>>,
    #[derivative(Default(value = "Box::new(VT102Emulation::new(None))"))]
    emulation: Box<VT102Emulation>,
    echos: VecDeque<IpcEvent>,
}

impl Shell {
    #[inline]
    pub fn set_terminal_size(&mut self, cols: i32, rows: i32) {
        self.emulation
            .emulation_mut()
            .current_screen_mut()
            .resize_image(rows, cols);
    }

    #[inline]
    pub fn insert_command(&mut self, name: String, command: Gd<Command>) {
        self.command_map.insert(name, command);
    }

    /// Get current cursor position, represent as (row, column)
    ///
    /// The origin point of cursor is (1, 1)
    #[inline]
    pub fn get_cursor_position(&self) -> Vector2i {
        let screen = self.emulation.emulation().current_screen();
        Vector2i::new(screen.get_cursor_y() + 1, screen.get_cursor_x() + 1)
    }

    #[inline]
    pub fn set_prompt(&mut self, host_name: &GString) {
        self.prompt = format!("\r\n{}> \u{200B}", host_name);
    }

    #[inline]
    pub fn next_echo(&mut self) -> Option<IpcEvent> {
        self.echos.pop_front()
    }

    pub fn receive_char(&mut self, c: wchar_t) {
        self.emulation.receive_char(c);

        match c {
            ASCII_ESCAPE => {
                self.reset();
                self.argv[0] = ASCII_ESCAPE;
            }
            ASCII_LEFT_SQUARE_BRACKET => {
                if self.argv[0] != 0 {
                    self.argv[1] = ASCII_LEFT_SQUARE_BRACKET;
                } else {
                    self.extend(c);
                }
            }
            KEY_HOME => {
                if self.argv[1] != 0 {
                    self.reset();
                } else {
                    self.extend(c);
                }
            }
            KEY_END => {
                if self.argv[1] != 0 {
                    self.reset();
                } else {
                    self.extend(c);
                }
            }
            KEY_LEFT => {}
            KEY_RIGHT => {}
            KEY_UP => {}
            KEY_DOWN => {}
            ASCII_CARRIAGE_RETURN => {}
            ASCII_BACKSPACE => {}
            ASCII_TAB => {}
            _ => {
                if is_printable(c) {
                    self.extend(c);
                }
            }
        }
    }
}

/// Private functions:
impl Shell {
    fn extend(&mut self, c: wchar_t) {}

    fn excute_command(&mut self, data: String) {
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
            None => return,
        };
        if let Some(gd) = self.command_map.get(command) {
            Command::start(gd.clone(), params);
        } else {
            let send_back = format!(
                "`{}` is not recognized as an internal command.{}",
                command, self.prompt,
            );

            self.echos.extend(IpcEvent::pack_data(send_back));
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.argv[0] = 0;
        self.argv[1] = 0;
    }
}
