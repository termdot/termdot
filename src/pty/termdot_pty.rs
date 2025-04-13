use crate::{
    events::{EventBus, Events},
    terminal_version,
};
use ipc::{ipc_channel::IpcChannel, ipc_event::IpcEvent, HEART_BEAT_INTERVAL};
use std::{path::PathBuf, time::Instant};
use termio::{
    cli::session::SessionPropsId,
    emulator::pty::{Pty, PtySignals},
};
use tlib::{log::error, object::ObjectSubclass};
use tmui::prelude::*;

#[extends(Object)]
#[async_task(name = "AsyncTask", value = "i32")]
pub struct TermdotPty {
    #[derivative(Default(value = "true"))]
    writeable: bool,
    flow_control_enable: bool,
    window_size: Size,
    working_directory: PathBuf,
    utf8_mode: bool,
    running: bool,
    timeout: u32,
    ipc_context: Option<IpcChannel>,
    last_heart_beat: Option<Instant>,
}

impl ObjectSubclass for TermdotPty {
    const NAME: &'static str = "TermdotPty";
}

impl ObjectImpl for TermdotPty {}

impl Pty for TermdotPty {
    fn start(&mut self, id: SessionPropsId, _: &str, _: Vec<&str>, _: Vec<&str>) -> bool {
        self.running = true;

        self.ipc_context = IpcChannel::terminal(id);

        self.running
    }

    fn set_writeable(&mut self, writeable: bool) {
        self.writeable = writeable
    }

    fn writeable(&self) -> bool {
        self.writeable
    }

    fn set_flow_control_enable(&mut self, on: bool) {
        self.flow_control_enable = on;
    }

    fn flow_control_enable(&self) -> bool {
        self.flow_control_enable
    }

    fn set_window_size(&mut self, cols: i32, rows: i32) {
        self.window_size = Size::new(cols, rows);
        self.send_ipc_data(IpcEvent::SetTerminalSize(cols, rows));
    }

    fn window_size(&self) -> Size {
        self.window_size
    }

    fn set_working_directory(&mut self, directory: PathBuf) {
        self.working_directory = directory
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn set_utf8_mode(&mut self, on: bool) {
        self.utf8_mode = on;
    }

    fn set_timeout(&mut self, timeout: u32) {
        self.timeout = timeout;
    }

    fn send_data(&mut self, data: String) {
        if !self.writeable {
            return;
        }

        let packed_data = IpcEvent::pack_data(&data);
        for chunk in packed_data {
            self.send_ipc_data(chunk);
        }
    }

    fn read_data(&mut self) -> Vec<u8> {
        if !self.running {
            return vec![];
        }

        let ctx = match self.ipc_context.as_ref() {
            Some(ctx) => ctx,
            None => return vec![],
        };

        if let Some(last_heart_beat) = self.last_heart_beat {
            if last_heart_beat.elapsed().as_millis() > HEART_BEAT_INTERVAL * 10 {
                EventBus::push(Events::HeartBeatUndetected);
                self.running = false;
                return vec![];
            }
        }

        if let Some(evt) = ctx.try_recv() {
            match evt {
                IpcEvent::HeartBeat => self.last_heart_beat = Some(Instant::now()),
                IpcEvent::Ready => {
                    EventBus::push(Events::MasterReay);
                    self.send_ipc_data(IpcEvent::SetTerminalSize(
                        self.window_size.width(),
                        self.window_size.height(),
                    ));
                    self.send_ipc_data(IpcEvent::pack_terminal_version(terminal_version()));
                }
                IpcEvent::Exit => {
                    self.running = false;
                    self.ipc_context = None;
                }
                IpcEvent::SetTerminalSize(_, _) => {}
                IpcEvent::SendData(data, len) => {
                    let mut data = data.to_vec();
                    data.truncate(len);
                    return data;
                }
                IpcEvent::HostNameChanged(data, len) => {
                    let mut data = data.to_vec();
                    data.truncate(len);
                    let data = match String::from_utf8(data) {
                        Ok(d) => d,
                        Err(e) => {
                            panic!(
                                "[Termdot::recv_data] Parse utf-8 string failed, err = {:?}",
                                e
                            );
                        }
                    };
                    EventBus::push(Events::TitleChanged(data));
                }
                _ => {}
            }
        }

        vec![]
    }

    fn on_window_closed(&mut self) {
        self.send_ipc_data(IpcEvent::Exit);
    }
}

impl TermdotPty {
    #[inline]
    pub fn send_ipc_data(&self, evt: IpcEvent) {
        if let Some(ctx) = self.ipc_context.as_ref() {
            if let Err(e) = ctx.try_send(evt) {
                error!("IPC send data failed, err = {:?}", e)
            }
        }
    }
}

impl PtySignals for TermdotPty {}

impl TermdotPty {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
