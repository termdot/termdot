use ipc::{ipc_context::IpcContext, ipc_event::IpcEvent};
use std::path::PathBuf;
use termio::{
    cli::session::SessionPropsId,
    emulator::pty::{Pty, PtySignals},
};
use tlib::{log::error, object::ObjectSubclass};
use tmui::prelude::*;

#[extends(Object)]
pub struct TermdotPty {
    #[derivative(Default(value = "true"))]
    writeable: bool,
    flow_control_enable: bool,
    window_size: Size,
    working_directory: PathBuf,
    utf8_mode: bool,
    running: bool,
    timeout: u32,
    ipc_context: Option<IpcContext>,
}

impl ObjectSubclass for TermdotPty {
    const NAME: &'static str = "TermdotPty";
}

impl ObjectImpl for TermdotPty {}

impl Pty for TermdotPty {
    fn start(
        &mut self,
        _id: SessionPropsId,
        _program: &str,
        _arguments: Vec<&str>,
        _enviroment: Vec<&str>,
    ) -> bool {
        self.running = true;

        self.ipc_context = IpcContext::slave();

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

        let packed_data = IpcEvent::pack_data(data);
        for chunk in packed_data {
            if let Some(ctx) = self.ipc_context.as_ref() {
                if let Err(e) = ctx.try_send(chunk) {
                    error!("IPC send data failed, err = {:?}", e)
                }
            }
        }
    }

    fn read_data(&mut self) -> Vec<u8> {
        let ctx = match self.ipc_context.as_ref() {
            Some(ctx) => ctx,
            None => return vec![],
        };

        if let Some(evt) = ctx.try_recv() {
            match evt {
                IpcEvent::Exit => {}
                IpcEvent::SetTerminalSize(_, _) => {}
                IpcEvent::SendData(data, len) => {
                    let mut data = data.to_vec();
                    data.truncate(len);
                    return data;
                }
            }
        }

        vec![]
    }
}

impl PtySignals for TermdotPty {}

impl TermdotPty {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
