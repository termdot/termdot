use crate::command::execute_status::ShExecuteStatus;
use crate::command::internal::cls::CmdCls;
use crate::command::internal::log::CmdLog;
use crate::command::internal::version::CmdVersion;
use crate::command::internal::{IInternalCommand, InternalCommandHnd};
use crate::command::{Command, internal::InternalCommand};
use crate::utils::ansi_string::godot::AnsiString;
use crate::utils::ansi_string::rust::ShAnsiString;
use crate::utils::charmap::*;
use crate::utils::escape_sequence::ESC0M;
use ahash::AHashMap;
use derivative::Derivative;
use godot::{
    builtin::{GString, Vector2i, array},
    obj::Gd,
};
use ipc::ipc_event::IpcEvent;
use termio::emulator::core::uwchar_t;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ptr::NonNull;
use std::{collections::VecDeque, str::FromStr};
use termio::emulator::emulation::{Emulation, VT102Emulation};
use tmui::tlib::global::SemanticExt;
use tmui::tlib::ptr_mut;
use wchar::{wch, wchar_t};
use widestring::WideString;

thread_local! {
    pub static SHELL: RefCell<Option<NonNull<Shell>>> = const { RefCell::new(None) };
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct Shell {
    prompt: String,
    buffer: Vec<wchar_t>,
    cursor: usize,
    is_executing: bool,
    u_stack: Vec<Vec<wchar_t>>,
    d_stack: Vec<Vec<wchar_t>>,
    /// (Cols, Rows)
    cursor_origin: Vector2i,
    columns: i32,
    argv: [wchar_t; 2],

    internal_command_map: AHashMap<String, InternalCommand>,
    command_map: AHashMap<String, Gd<Command>>,
    #[derivative(Default(value = "Box::new(VT102Emulation::new(None))"))]
    emulation: Box<VT102Emulation>,
    echos: VecDeque<IpcEvent>,

    running_command: Option<Gd<Command>>,
    running_internal_command: Option<InternalCommandHnd>,
}

impl Shell {
    #[inline]
    pub fn init(&mut self) {
        SHELL.with(|rf| *rf.borrow_mut() = NonNull::new(self));
    }

    #[inline]
    /// Get current terminal size, represent as (cols, rows)
    pub fn get_terminal_size(&self) -> Vector2i {
        let screen = self.emulation.emulation().current_screen();
        Vector2i::new(screen.get_columns(), screen.get_lines())
    }

    #[inline]
    /// Get current cursor position, represent as (cols, rows)
    ///
    /// The origin point of cursor is (1, 1)
    pub fn get_cursor_position(&self) -> Vector2i {
        let screen = self.emulation.emulation().current_screen();
        Vector2i::new(screen.get_cursor_x() + 1, screen.get_cursor_y() + 1)
    }

    #[inline]
    pub fn echo(&mut self, text: Gd<AnsiString>) {
        let text = text.bind().as_str().to_string();
        self.echos.extend(IpcEvent::pack_data(&text));

        let wstr = WideString::from_str(&text);
        for &c in wstr.as_slice() {
            #[allow(clippy::useless_transmute)]
            let c: wchar_t = unsafe { std::mem::transmute(c) };
            self.emulation.receive_char(c);
        }
    }

    #[inline]
    pub fn sh_echo(&mut self, text: ShAnsiString) {
        let text = text.as_str().to_string();
        self.echos.extend(IpcEvent::pack_data(&text));

        let wstr = WideString::from_str(&text);
        for &c in wstr.as_slice() {
            #[allow(clippy::useless_transmute)]
            let c: wchar_t = unsafe { std::mem::transmute(c) };
            self.emulation.receive_char(c);
        }
    }

    #[inline]
    pub fn insert_command(&mut self, name: String, command: Gd<Command>) {
        self.command_map.insert(name, command);
    }

    #[inline]
    pub fn init_internal_command(&mut self) {
        let cmd = CmdCls.boxed();
        self.internal_command_map.insert(cmd.command_name(), cmd);

        let cmd = CmdVersion.boxed();
        self.internal_command_map.insert(cmd.command_name(), cmd);

        let cmd = CmdLog.boxed();
        self.internal_command_map.insert(cmd.command_name(), cmd);
    }

    #[inline]
    pub fn set_terminal_size(&mut self, cols: i32, rows: i32) {
        self.emulation.emulation_mut().set_image_size(rows, cols);
    }

    #[inline]
    pub fn set_prompt(&mut self, host_name: &GString) {
        self.prompt = format!("{}> \u{200B}", host_name);
    }

    #[inline]
    pub fn prompt(&mut self) {
        let prompt = format!("{}{}", ESC0M, self.prompt);
        let wstr = WideString::from_str(&prompt);
        for &c in wstr.as_slice() {
            #[allow(clippy::useless_transmute)]
            let c: wchar_t = unsafe { std::mem::transmute(c) };
            self.emulation.receive_char(c);
        }
        self.echos.extend(IpcEvent::pack_data(&prompt));
        self.cursor_origin = self.get_cursor_position();
        self.columns = self.get_terminal_size().x;
    }

    #[inline]
    pub fn crlf_prompt(&mut self) {
        let prompt = format!("{}\r\n{}", ESC0M, self.prompt);
        let wstr = WideString::from_str(&prompt);
        for &c in wstr.as_slice() {
            #[allow(clippy::useless_transmute)]
            let c: wchar_t = unsafe { std::mem::transmute(c) };
            self.emulation.receive_char(c);
        }
        self.echos.extend(IpcEvent::pack_data(&prompt));
        self.cursor_origin = self.get_cursor_position();
        self.columns = self.get_terminal_size().x;
    }

    #[inline]
    pub fn next_echo(&mut self) -> Option<IpcEvent> {
        self.echos.pop_front()
    }

    #[inline]
    pub fn process_running_command(&mut self) {
        if let Some(icmd) = self.running_internal_command {
            if ptr_mut!(icmd).running() == ShExecuteStatus::Done {
                self.running_internal_command = None;
                self.crlf_prompt();
            }
        }

        if let Some(gd) = self.running_command.clone() {
            if Command::running(gd) == ShExecuteStatus::Done {
                self.running_command = None;
                self.crlf_prompt();
            }
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.emulation.reset();
    }

    #[inline]
    pub fn get_emulation(&self) -> &VT102Emulation {
        self.emulation.as_ref()
    }

    #[inline]
    pub fn get_emulation_mut(&mut self) -> &mut VT102Emulation {
        self.emulation.as_mut()
    }

    pub fn receive_char(&mut self, c: wchar_t) {
        let oc = match c {
            CTL_ESCAPE => {
                self.reset_argv();
                self.argv[0] = CTL_ESCAPE;
                Some(c)
            }
            ASCII_LEFT_SQUARE_BRACKET => {
                if self.argv[0] == CTL_ESCAPE {
                    self.argv[1] = ASCII_LEFT_SQUARE_BRACKET;
                } else {
                    self.extend(c);
                }
                Some(c)
            }
            KEY_HOME => {
                if self.argv[1] == ASCII_LEFT_SQUARE_BRACKET {
                    self.cursor = 0;
                    self.emulation.receive_char(wch!(';'));
                    self.map_set_cursor();
                    self.report_cursor();
                    None
                } else {
                    self.extend(c);
                    Some(c)
                }
            }
            KEY_END => {
                if self.argv[1] == ASCII_LEFT_SQUARE_BRACKET {
                    self.cursor = self.buffer.len();
                    self.emulation.receive_char(wch!(';'));
                    self.map_set_cursor();
                    self.report_cursor();
                    None
                } else {
                    self.extend(c);
                    Some(c)
                }
            }
            KEY_LEFT => {
                if self.argv[1] == ASCII_LEFT_SQUARE_BRACKET {
                    if self.cursor != 0 {
                        self.cursor -= 1;
                        self.report_cursor();
                        Some(c)
                    } else {
                        self.emulation.receive_char(wch!(';'));
                        self.map_set_cursor();
                        None
                    }
                } else {
                    self.extend(c);
                    Some(c)
                }
            }
            KEY_RIGHT => {
                if self.argv[1] == ASCII_LEFT_SQUARE_BRACKET {
                    if self.cursor < self.buffer.len() {
                        self.cursor += 1;
                        self.report_cursor();
                        Some(c)
                    } else {
                        self.emulation.receive_char(wch!(';'));
                        self.map_set_cursor();
                        None
                    }
                } else {
                    self.extend(c);
                    Some(c)
                }
            }
            KEY_UP => {
                if self.argv[1] == ASCII_LEFT_SQUARE_BRACKET {
                    if let Some(u_pop) = self.u_stack.pop() {
                        if !self.buffer.is_empty() {
                            self.d_stack.push(self.buffer.clone());
                        }
                        self.buffer = u_pop;
                        self.cursor = self.buffer.len();

                        let replay_text = self.replay_text();
                        self.echos
                            .extend(IpcEvent::pack_data(&replay_text.to_string_lossy()));

                        for &c in replay_text.as_slice() {
                            #[allow(clippy::useless_transmute)]
                            let c: wchar_t = unsafe { std::mem::transmute(c) };
                            self.emulation.receive_char(c);
                        }
                    }
                    self.emulation.receive_char(wch!(';'));
                    self.map_set_cursor();
                    None
                } else {
                    self.extend(c);
                    Some(c)
                }
            }
            KEY_DOWN => {
                if self.argv[1] == ASCII_LEFT_SQUARE_BRACKET {
                    if let Some(d_pop) = self.d_stack.pop() {
                        if !self.buffer.is_empty() {
                            self.u_stack.push(self.buffer.clone());
                        }
                        self.buffer = d_pop;
                        self.cursor = self.buffer.len();

                        let replay_text = self.replay_text();
                        self.echos
                            .extend(IpcEvent::pack_data(&replay_text.to_string_lossy()));

                        for &c in replay_text.as_slice() {
                            #[allow(clippy::useless_transmute)]
                            let c: wchar_t = unsafe { std::mem::transmute(c) };
                            self.emulation.receive_char(c);
                        }
                    }
                    self.emulation.receive_char(wch!(';'));
                    self.map_set_cursor();
                    None
                } else {
                    self.extend(c);
                    Some(c)
                }
            }
            CTL_BACKSPACE => {
                if self.cursor != 0 {
                    self.cursor -= 1;
                    self.buffer.remove(self.cursor);

                    let (row, col) = self.cursor_to_position();
                    let echo = format!("\x1B[{};{}H\x1B[K", row, col);
                    self.echos.extend(IpcEvent::pack_data(&echo));

                    Some(c)
                } else {
                    None
                }
            }
            CTL_TAB => {
                self.command_completion();
                None
            }
            CTL_SIGINT => {
                self.interrupt();
                None
            }
            CTL_CARRIAGE_RETURN => {
                #[allow(clippy::useless_transmute)]
                let buffer: Vec<uwchar_t> = unsafe { std::mem::transmute(self.buffer.clone()) }; 
                let data = WideString::from_vec(buffer).to_string_lossy();
                if !self.buffer.is_empty() {
                    self.u_stack.push(self.buffer.clone());
                }
                self.buffer.clear();
                self.cursor = 0;

                self.execute_command(&data);

                None
            }
            _ => {
                if is_printable(c) {
                    self.extend(c);
                }
                Some(c)
            }
        };

        if let Some(c) = oc {
            self.emulation.receive_char(c);
        }

        if c != CTL_ESCAPE && c != ASCII_LEFT_SQUARE_BRACKET {
            self.reset_argv();
        }
    }
}

/// Private functions:
impl Shell {
    #[inline]
    fn extend(&mut self, c: wchar_t) {
        self.buffer.insert(self.cursor, c);
        self.cursor += 1;

        if self.cursor == self.buffer.len() {
            let c: uwchar_t = unsafe { std::mem::transmute(c) };
            let c = WideString::from_vec(vec![c]).to_string_lossy();
            self.echos.extend(IpcEvent::pack_data(&c));
        } else {
            #[allow(clippy::useless_transmute)]
            let buffer: Vec<uwchar_t> = unsafe { std::mem::transmute(self.buffer.clone()) };
            let data = WideString::from_vec(buffer).to_string_lossy();
            let (row, col) = self.cursor_to_position();
            let echo = format!(
                "\x1B[{};{}H\x1B[K{}\x1B[{};{}H",
                self.cursor_origin.y, self.cursor_origin.x, data, row, col
            );
            self.echos.extend(IpcEvent::pack_data(&echo));
        }
    }

    fn execute_command(&mut self, data: &str) {
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

        self.next_line();
        if let Some(icmd) = self.internal_command_map.get_mut(command) {
            self.is_executing = true;
            match icmd.start(params) {
                ShExecuteStatus::Done => self.prompt(),
                ShExecuteStatus::Running => self.running_internal_command = Some(icmd.as_mut()),
            }
        } else if let Some(gd) = self.command_map.get(command) {
            self.is_executing = true;
            let gd = gd.clone();

            match Command::start(gd.clone(), params) {
                ShExecuteStatus::Done => self.crlf_prompt(),
                ShExecuteStatus::Running => self.running_command = Some(gd),
            }
        } else {
            self.is_executing = false;
            let send_back = if data.is_empty() {
                self.prompt.to_string()
            } else {
                format!(
                    "`{}` is not recognized as an internal or external command.\r\n{}",
                    command, self.prompt,
                )
            };

            let wstr = WideString::from_str(&send_back);
            for &c in wstr.as_slice() {
                #[allow(clippy::useless_transmute)]
                let c: wchar_t = unsafe { std::mem::transmute(c) };
                self.emulation.receive_char(c);
            }

            self.cursor_origin = self.get_cursor_position();
            self.columns = self.get_terminal_size().x;

            self.echos.extend(IpcEvent::pack_data(&send_back));
        }
    }

    #[inline]
    fn reset_argv(&mut self) {
        self.argv[0] = 0;
        self.argv[1] = 0;
    }

    fn report_cursor(&mut self) {
        let (row, col) = self.cursor_to_position();
        let echo = format!("\x1B[{};{}H", row, col);
        self.echos.extend(IpcEvent::pack_data(&echo));
    }

    fn replay_text(&self) -> WideString {
        let cursor_origin = self.cursor_origin;
        let text = format!("\x1B[{};{}H\x1B[K", cursor_origin.y, cursor_origin.x,);
        let mut text = WideString::from_str(&text);

        #[allow(clippy::useless_transmute)]
        let buffer: Vec<uwchar_t> = unsafe { std::mem::transmute(self.buffer.to_vec()) };
        let mut cur_text = WideString::from_vec(buffer).to_string_lossy();
        let cursor_pos = self.cursor_to_position();
        cur_text.push_str(&format!("\x1B[{};{}H", cursor_pos.0, cursor_pos.1));

        text.push_str(&cur_text);

        text
    }

    #[inline]
    fn cursor_to_position(&self) -> (i32, i32) {
        let row = (self.cursor as i32 + self.cursor_origin.x) / (self.columns + 1);
        let col = (self.cursor as i32 + self.cursor_origin.x) % (self.columns + 1);
        (self.cursor_origin.y + row, col)
    }

    #[inline]
    fn map_set_cursor(&mut self) {
        let (row, col) = self.cursor_to_position();
        let screen = self.emulation.emulation_mut().current_screen_mut();
        screen.set_cursor_y(row);
        screen.set_cursor_x(col);
    }

    fn command_completion(&mut self) {
        #[allow(clippy::useless_transmute)]
        let buffer: Vec<uwchar_t> = unsafe { std::mem::transmute(self.buffer.to_vec()) };
        let input = WideString::from_vec(buffer).to_string_lossy();
        let mut echo = String::new();
        let mut prompt = false;
        if input.is_empty() {
            let commands: Vec<&str> = self.command_map.keys().map(|c| c.as_str()).collect();
            if commands.is_empty() {
                let cursor_pos = self.get_cursor_position();
                echo.push_str(&format!("\x1B[{};{}H", cursor_pos.y, cursor_pos.x));
            } else {
                echo.push_str(&self.format_commands_list(&commands));
                prompt = true;
            }
        } else if input.len() != self.cursor {
            // Do nothing
        } else {
            let mut commands: Vec<&str> = self
                .command_map
                .keys()
                .filter_map(|cmd| {
                    if cmd.starts_with(&input) {
                        Some(cmd.as_str())
                    } else {
                        None
                    }
                })
                .collect();

            match commands.len().cmp(&1) {
                Ordering::Greater => {
                    echo.push_str(&self.format_commands_list(&commands));
                    prompt = true;
                }
                Ordering::Equal => {
                    let origin = self.cursor_origin;
                    let cmd = commands.pop().unwrap();
                    echo.push_str(&format!("\x1B[{};{}H{}", origin.y, origin.x, cmd));

                    let wstr = WideString::from_str(cmd);
                    #[allow(clippy::useless_transmute)]
                    let buffer: Vec<wchar_t> = unsafe { std::mem::transmute(wstr.as_slice().to_vec()) };
                    self.buffer = buffer;
                    self.cursor = self.buffer.len();
                }
                Ordering::Less => {}
            }
        }

        if prompt {
            echo.push_str(&format!("\r\n{}", self.prompt));
        }

        if !echo.is_empty() {
            for e in IpcEvent::pack_data(&echo) {
                self.echos.push_back(e);
            }

            let wstr = WideString::from_str(&echo);
            for &c in wstr.as_slice() {
                #[allow(clippy::useless_transmute)]
                let c: wchar_t = unsafe { std::mem::transmute(c) };
                self.emulation.receive_char(c);
            }

            if prompt {
                self.cursor_origin = self.get_cursor_position();

                if input.is_empty() {
                    return;
                }

                for e in IpcEvent::pack_data(&input) {
                    self.echos.push_back(e);
                }

                let wstr = WideString::from_str(&input);
                for &c in wstr.as_slice() {
                    #[allow(clippy::useless_transmute)]
                    let c: wchar_t = unsafe { std::mem::transmute(c) };
                    self.emulation.receive_char(c);
                }
            }
        }
    }

    fn format_commands_list(&self, commands: &[&str]) -> String {
        let size = self.get_terminal_size();
        let width = size.x as usize;
        let mut current_width = 0;

        let mut echo = "\r\n".to_string();
        for cmd in commands {
            let cmd_len = cmd.len() + 2;

            if current_width + cmd_len > width {
                echo.push_str("\r\n");
                current_width = 0;
            }

            echo.push_str(&format!("{}  ", cmd));
            current_width += cmd_len;
        }
        echo
    }

    #[inline]
    fn next_line(&mut self) {
        let send_back = "\r\n";
        self.echos
            .push_back(IpcEvent::pack_data(send_back).pop().unwrap());
        let wstr = WideString::from_str(&send_back);
        for &c in wstr.as_slice() {
            #[allow(clippy::useless_transmute)]
            let c: wchar_t = unsafe { std::mem::transmute(c) };
            self.emulation.receive_char(c);
        }
        self.cursor_origin = self.get_cursor_position();
    }

    #[inline]
    fn interrupt(&mut self) {
        let mut interrupted = false;
        if let Some(running_command) = self.running_command.clone() {
            Command::interrupting(running_command);

            self.running_command = None;
            interrupted = true;
        }

        if let Some(icm) = self.running_internal_command {
            ptr_mut!(icm).interrupting();

            self.running_internal_command = None;
            interrupted = true;
        }

        if interrupted {
            self.sh_echo(ShAnsiString::new().append("\r\n^C"));
        }

        self.crlf_prompt();
    }
}
