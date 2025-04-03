pub mod ipc_context;
pub mod ipc_event;

pub const MEM_QUEUE_MASTER: &str = "/tmdt_m_q";
pub const MEM_QUEUE_SLAVE: &str = "/tmdt_s_q";
pub const MEM_SIGNAL: &str = "/tmdt_si";
pub const IPC_DATA_SIZE: usize = 512;

pub const HEART_BEAT_INTERVAL: u128 = 100;
