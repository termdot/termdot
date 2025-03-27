use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use crate::{IPC_DATA_SIZE, MEM_QUEUE_MASTER, MEM_QUEUE_SLAVE, MEM_SIGNAL, ipc_event::IpcEvent};
use godot::global::godot_error;
use log::error;
use tmui::tipc::{
    mem::{
        BuildType,
        mem_queue::{MemQueue, MemQueueBuilder, MemQueueError},
    },
    raw_sync::{
        Timeout,
        events::{Event, EventInit, EventState},
    },
    shared_memory::{Shmem, ShmemConf},
};

pub static SHARED_ID: AtomicU64 = AtomicU64::new(0);

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ContextRole {
    Master,
    Slave,
}

pub struct IpcContext {
    role: ContextRole,
    master_queue: MemQueue<IPC_DATA_SIZE, IpcEvent>,
    slave_queue: MemQueue<IPC_DATA_SIZE, IpcEvent>,
    signal: Shmem,
}

impl IpcContext {
    pub fn master() -> Option<Self> {
        let id = SHARED_ID.load(Ordering::Relaxed);
        let (a, b, c) = (
            format!("{}_{}", MEM_QUEUE_MASTER, id),
            format!("{}_{}", MEM_QUEUE_SLAVE, id),
            format!("{}_{}", MEM_SIGNAL, id),
        );
        Some(Self {
            role: ContextRole::Master,
            master_queue: MemQueueBuilder::new()
                .build_type(BuildType::Create)
                .os_id(a)
                .build()
                .inspect_err(|e| {
                    godot_error!("[IpcContext::master] Master `MemQueue` create error, create `IpcContext` failed, e = {:?}", e)
                })
                .ok()?,
            slave_queue: MemQueueBuilder::new()
                .build_type(BuildType::Create)
                .os_id(b)
                .build()
                .inspect_err(|e| {
                    godot_error!("[IpcContext::master] Slave `MemQueue` create error, create `IpcContext` failed, e = {:?}", e)
                })
                .ok()?,
            signal: ShmemConf::new().os_id(c).size(size_of::<Event>()).create().inspect_err(|e| {
                godot_error!("[IpcContext::master] Wait signal create error, create `IpcContext` failed, e = {:?}", e)
            }).ok()?,
        })
    }

    pub fn slave() -> Option<Self> {
        let id = SHARED_ID.load(Ordering::Relaxed);
        let (a, b, c) = (
            format!("{}_{}", MEM_QUEUE_MASTER, id),
            format!("{}_{}", MEM_QUEUE_SLAVE, id),
            format!("{}_{}", MEM_SIGNAL, id),
        );
        Some(Self {
            role: ContextRole::Slave,
            master_queue: MemQueueBuilder::new()
                .build_type(BuildType::Open)
                .os_id(a)
                .build()
                .inspect_err(|e| error!("[IpcContext::slave] Master `MemQueue` open error, create `IpcContext` failed, e = {:?}", e))
                .ok()?,
            slave_queue: MemQueueBuilder::new()
                .build_type(BuildType::Open)
                .os_id(b)
                .build()
                .inspect_err(|e| error!("[IpcContext::slave] Slave `MemQueue` open error, create `IpcContext` failed, e = {:?}", e))
                .ok()?,
            signal: ShmemConf::new().os_id(c).open().inspect_err(|e| {
                godot_error!("[IpcContext::slave] Wait signal open error, create `IpcContext` failed, e = {:?}", e)
            }).ok()?,
        })
    }

    #[inline]
    pub fn try_send(&self, evt: IpcEvent) -> Result<(), MemQueueError> {
        match self.role {
            ContextRole::Master => self.master_queue.try_write(evt),
            ContextRole::Slave => self.slave_queue.try_write(evt),
        }
    }

    #[inline]
    pub fn try_recv(&self) -> Option<IpcEvent> {
        match self.role {
            ContextRole::Master => self.slave_queue.try_read(),
            ContextRole::Slave => self.master_queue.try_read(),
        }
    }

    #[inline]
    fn wait_signaled(&self, timeout: Timeout) {
        if let Ok((evt, _)) = unsafe { Event::new(self.signal.as_ptr(), true) } {
            if let Err(e) = evt.wait(timeout) {
                godot_error!("[IpcContext::wait_signaled] Error occurred, {:?}", e)
            }
        }
    }

    #[inline]
    fn signaled(&self) {
        if let Ok((evt, _)) = unsafe { Event::from_existing(self.signal.as_ptr()) } {
            if let Err(e) = evt.set(EventState::Signaled) {
                error!("[IpcContext::signaled] Error occurred, {:?}", e)
            }
        }
    }
}

impl Drop for IpcContext {
    #[inline]
    fn drop(&mut self) {
        match self.role {
            ContextRole::Master => {
                self.wait_signaled(Timeout::Val(Duration::from_secs(1)));
            }
            ContextRole::Slave => self.signaled(),
        }
    }
}
