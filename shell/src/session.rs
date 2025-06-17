use crate::shell::Shell;
use common::gb_error;
use ipc::{ipc_channel::IpcChannel, ipc_event::IpcEvent};
use termio::cli::session::SessionPropsId;
use tmui::tlib::utils::SnowflakeGuidGenerator;

pub struct Session {
    id: SessionPropsId,
    shell: Shell,
    ipc_channel: IpcChannel,
}

impl Session {
    #[inline]
    pub fn new() -> Option<Self> {
        let id = SnowflakeGuidGenerator::next_id().expect("[Session::new] Generate guid failed.");
        Some(Self {
            id,
            shell: Shell::default(),
            ipc_channel: IpcChannel::shell(id)?,
        })
    }

    #[inline]
    pub fn id(&self) -> SessionPropsId {
        self.id
    }

    #[inline]
    pub fn set_prompt(&mut self, host_name: &str) {
        self.shell.set_prompt(host_name);
    }

    #[inline]
    pub fn send_ipc_event(&self, event: IpcEvent) {
        if let Err(e) = self.ipc_channel.try_send(event) {
            gb_error!(
                "[Session::send_ipc_event] Send ipc event failed, e = {:?}",
                e
            );
        }
    }

    #[inline]
    pub fn process_running_command(&mut self) {
        self.shell.process_running_command();
    }
}
