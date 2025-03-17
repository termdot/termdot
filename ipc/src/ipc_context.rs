use crate::{IPC_DATA_SIZE, MEM_QUEUE_NAME, ipc_event::IpcEvent};
use godot::global::godot_error;
use log::error;
use tmui::tipc::mem::{
    BuildType,
    mem_queue::{MemQueue, MemQueueBuilder, MemQueueError},
};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ContextRole {
    Master,
    Slave,
}

pub struct IpcContext {
    role: ContextRole,
    mem_queue: MemQueue<IPC_DATA_SIZE, IpcEvent>,
}

impl IpcContext {
    #[inline]
    pub fn master() -> Option<Self> {
        Some(Self {
            role: ContextRole::Master,
            mem_queue: MemQueueBuilder::new()
                .build_type(BuildType::Create)
                .os_id(MEM_QUEUE_NAME)
                .build()
                .inspect_err(|e| {
                    godot_error!("[IpcContext::master] `MemQueue` create error, create `IpcContext` failed, e = {:?}", e)
                })
                .ok()?,
        })
    }

    #[inline]
    pub fn slave() -> Option<Self> {
        Some(Self {
            role: ContextRole::Slave,
            mem_queue: MemQueueBuilder::new()
                .build_type(BuildType::Open)
                .os_id(MEM_QUEUE_NAME)
                .build()
                .inspect_err(|e| error!("[IpcContext::slave] MemQueue open error, create `IpcContext` failed, e = {:?}", e))
                .ok()?,
        })
    }

    #[inline]
    pub fn get_role(&self) -> ContextRole {
        self.role
    }

    #[inline]
    pub fn try_send(&self, evt: IpcEvent) -> Result<(), MemQueueError> {
        self.mem_queue.try_write(evt)
    }

    #[inline]
    pub fn has_event(&self) -> bool {
        self.mem_queue.has_event()
    }

    #[inline]
    pub fn try_recv(&self) -> Option<IpcEvent> {
        self.mem_queue.try_read()
    }
}
