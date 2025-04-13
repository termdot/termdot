pub mod ipc_channel;
pub mod ipc_context;
pub mod ipc_event;

pub const MEM_QUEUE_REGISTER: &str = "/tmdt_reg";
pub const MEM_QUEUE_SHELL: &str = "/tmdt_sh";
pub const MEM_QUEUE_TERMINAL: &str = "/tmdt_tm";
pub const MEM_CTX: &str = "/tmdt_ctx";
pub const IPC_DATA_SIZE: usize = 2048;
pub const IPC_REGISTER_SIZE: usize = 64;

pub const HEART_BEAT_INTERVAL: u128 = 100;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IpcRole {
    Shell,
    Terminal,
}
