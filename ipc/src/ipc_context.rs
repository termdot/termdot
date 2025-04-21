use crate::{IPC_REGISTER_SIZE, IpcRole, MEM_CTX};
use log::error;
use termio::cli::session::SessionPropsId;
use tmui::tipc::{
    mem::{
        BuildType,
        mem_queue::{MemQueue, MemQueueBuilder, MemQueueError},
    },
    shared_memory::ShmemError,
};

pub struct IpcContext {
    role: IpcRole,
    queue: MemQueue<IPC_REGISTER_SIZE, SessionPropsId>,
}

impl IpcContext {
    pub fn shell() -> Option<Self> {
        Some(Self {
            role: IpcRole::Shell,
            queue: MemQueueBuilder::new()
                .build_type(BuildType::Open)
                .os_id(MEM_CTX)
                .build()
                .ok()?,
        })
    }

    pub fn terminal() -> Option<Self> {
        let mut clean_hint = false;
        let queue = match MemQueueBuilder::new()
            .build_type(BuildType::Create)
            .os_id(&MEM_CTX)
            .build()
        {
            Ok(mq) => mq,
            Err(e) => match e {
                ShmemError::MappingIdExists => {
                    clean_hint = true;
                    MemQueueBuilder::new()
                        .build_type(BuildType::Open)
                        .os_id(MEM_CTX)
                        .build()
                        .inspect_err(|e| error!("[IpcContext::terminal] `MemQueue` open error, create `IpcContext` failed, e = {:?}", e))
                        .ok()?
                }
                _ => {
                    error!(
                        "[IpcContext::terminal] `MemQueue` create error, create `IpcContext` failed, e = {:?}",
                        e
                    );
                    return None;
                }
            },
        };

        if clean_hint {}

        Some(Self {
            role: IpcRole::Terminal,
            queue,
        })
    }

    #[inline]
    pub fn try_send(&self, id: SessionPropsId) -> Result<(), MemQueueError> {
        match self.role {
            IpcRole::Shell => self.queue.try_write(id),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn try_recv(&self) -> Option<SessionPropsId> {
        match self.role {
            IpcRole::Terminal => self.queue.try_read(),
            _ => unreachable!(),
        }
    }
}
