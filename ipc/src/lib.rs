pub mod ipc_channel;
pub mod ipc_context;
pub mod ipc_event;
pub mod register_info;

pub const MEM_QUEUE_REGISTER: &str = "/t_reg";
pub const MEM_QUEUE_SHELL: &str = "/t_sh";
pub const MEM_QUEUE_TERMINAL: &str = "/t_tm";
pub const MEM_CTX: &str = "/t_ctx";
pub const MEM_TERMINAL_REGISTER: &str = "/t_tm_rg";
pub const MEM_SHELL_REGISTER: &str = "/t_sh_rg";
pub const MEM_SESSION_REGISTER: &str = "/t_sn_rg";
pub const IPC_DATA_SIZE: usize = 2048;
pub const IPC_REGISTER_SIZE: usize = 64;

pub const HEART_BEAT_INTERVAL: u128 = 100;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IpcRole {
    Shell,
    Terminal,
}
