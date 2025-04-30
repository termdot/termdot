use crate::{IPC_DATA_SIZE, IpcRole, MEM_QUEUE_SHELL, MEM_QUEUE_TERMINAL, ipc_event::IpcEvent};
use godot::global::godot_error;
use log::error;
use termio::cli::session::SessionPropsId;
use tmui::tipc::{
    mem::{
        BuildType,
        mem_queue::{MemQueue, MemQueueBuilder, MemQueueError},
    },
    shared_memory::ShmemError,
};

pub struct IpcChannel {
    role: IpcRole,
    /// Shell send, terminal receive
    shell_queue: MemQueue<IPC_DATA_SIZE, IpcEvent>,
    /// Terminal send, shell receive
    terminal_queue: MemQueue<IPC_DATA_SIZE, IpcEvent>,
}

impl IpcChannel {
    pub fn shell(id: SessionPropsId) -> Option<Self> {
        let (sn, tn) = (
            format!("{}_{}", MEM_QUEUE_SHELL, id),
            format!("{}_{}", MEM_QUEUE_TERMINAL, id),
        );
        let mut clean_hint = false;
        let shell_queue = match MemQueueBuilder::new()
            .build_type(BuildType::Create)
            .os_id(&sn)
            .build()
        {
            Ok(mq) => mq,
            Err(e) => match e {
                ShmemError::MappingIdExists => {
                    clean_hint = true;
                    MemQueueBuilder::new()
                        .build_type(BuildType::Open)
                        .os_id(sn)
                        .build()
                        .inspect_err(|e| error!("[IpcContext::shell] Shell `MemQueue` open error, create `IpcContext` failed, e = {:?}", e))
                        .ok()?
                }
                _ => {
                    godot_error!(
                        "[IpcContext::shell] Shell `MemQueue` create error, create `IpcContext` failed, e = {:?}",
                        e
                    );
                    return None;
                }
            },
        };
        if clean_hint {
            shell_queue.clear();
        }

        let mut clean_hint = false;
        let terminal_queue = match MemQueueBuilder::new()
            .build_type(BuildType::Create)
            .os_id(&tn)
            .build()
        {
            Ok(mq) => mq,
            Err(e) => match e {
                ShmemError::MappingIdExists => {
                    clean_hint = true;
                    MemQueueBuilder::new()
                        .build_type(BuildType::Open)
                        .os_id(tn)
                        .build()
                        .inspect_err(|e| error!("[IpcContext::shell] Terminal `MemQueue` open error, create `IpcContext` failed, e = {:?}", e))
                        .ok()?
                }
                _ => {
                    godot_error!(
                        "[IpcContext::shell] Terminal `MemQueue` create error, create `IpcContext` failed, e = {:?}",
                        e
                    );
                    return None;
                }
            },
        };
        if clean_hint {
            terminal_queue.clear();
        }

        Some(Self {
            role: IpcRole::Shell,
            shell_queue,
            terminal_queue,
        })
    }

    pub fn terminal(id: SessionPropsId) -> Option<Self> {
        let (sn, tn) = (
            format!("{}_{}", MEM_QUEUE_SHELL, id),
            format!("{}_{}", MEM_QUEUE_TERMINAL, id),
        );
        Some(Self {
            role: IpcRole::Terminal,
            shell_queue: MemQueueBuilder::new()
                .build_type(BuildType::Open)
                .os_id(sn)
                .build()
                .inspect_err(|e| error!("[IpcContext::terminal] Shell `MemQueue` open error, create `IpcContext` failed, e = {:?}", e))
                .ok()?,
            terminal_queue: MemQueueBuilder::new()
                .build_type(BuildType::Open)
                .os_id(tn)
                .build()
                .inspect_err(|e| error!("[IpcContext::terminal] Terminal `MemQueue` open error, create `IpcContext` failed, e = {:?}", e))
                .ok()?,
        })
    }

    #[inline]
    pub fn try_send(&self, evt: IpcEvent) -> Result<(), MemQueueError> {
        match self.role {
            IpcRole::Shell => self.shell_queue.try_write(evt),
            IpcRole::Terminal => self.terminal_queue.try_write(evt),
        }
    }

    #[inline]
    pub fn try_recv(&self) -> Option<IpcEvent> {
        match self.role {
            IpcRole::Shell => self.terminal_queue.try_read(),
            IpcRole::Terminal => self.shell_queue.try_read(),
        }
    }
}
