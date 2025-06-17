use crate::{
    IPC_REGISTER_SIZE, IpcRole, MEM_CTX, MEM_SESSION_REGISTER, MEM_SHELL_REGISTER,
    MEM_TERMINAL_REGISTER,
    register_info::{
        RegisterInfo,
        register_list::{RegisterList, RegisterListBuilder},
    },
};
use common::{gb_error, gb_warn, typedef::RegisterInfoId};
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
    terminal_register_list: RegisterList<IPC_REGISTER_SIZE, RegisterInfo>,
    shell_register_list: RegisterList<IPC_REGISTER_SIZE, RegisterInfo>,
    session_register_list: RegisterList<IPC_REGISTER_SIZE, RegisterInfo>,
    queue: MemQueue<IPC_REGISTER_SIZE, SessionPropsId>,
}

impl IpcContext {
    pub fn shell() -> Option<Self> {
        let terminal_register_list = Self::create_register_list(MEM_TERMINAL_REGISTER)?;
        let shell_register_list = Self::create_register_list(MEM_SHELL_REGISTER)?;
        let session_register_list = Self::create_register_list(MEM_SESSION_REGISTER)?;

        let queue = Self::create_mem_queue(MEM_CTX)?;

        Some(Self {
            role: IpcRole::Shell,
            terminal_register_list,
            shell_register_list,
            session_register_list,
            queue,
        })
    }

    pub fn terminal() -> Option<Self> {
        let terminal_register_list = Self::create_register_list(MEM_TERMINAL_REGISTER)?;
        let shell_register_list = Self::create_register_list(MEM_SHELL_REGISTER)?;
        let session_register_list = Self::create_register_list(MEM_SESSION_REGISTER)?;

        let queue = Self::create_mem_queue(MEM_CTX)?;

        Some(Self {
            role: IpcRole::Terminal,
            terminal_register_list,
            shell_register_list,
            session_register_list,
            queue,
        })
    }

    #[inline]
    fn create_mem_queue(os_id: &str) -> Option<MemQueue<IPC_REGISTER_SIZE, SessionPropsId>> {
        match MemQueueBuilder::new()
            .build_type(BuildType::Create)
            .os_id(os_id)
            .build()
        {
            Ok(mq) => Some(mq),
            Err(e) => match e {
                ShmemError::MappingIdExists => {
                    Some(MemQueueBuilder::new()
                        .build_type(BuildType::Open)
                        .os_id(os_id)
                        .build()
                        .inspect_err(|e| gb_error!("[IpcContext::create_mem_queue] `MemQueue` open error, create `IpcContext` failed, e = {:?}, os_id = {}", e, os_id))
                        .ok()?)
                }
                _ => {
                    gb_error!(
                        "[IpcContext::create_mem_queue] `MemQueue` create error, create `IpcContext` failed, e = {:?}, os_id = {}",
                        e, os_id
                    );
                    None
                }
            },
        }
    }

    #[inline]
    fn create_register_list(os_id: &str) -> Option<RegisterList<IPC_REGISTER_SIZE, RegisterInfo>> {
        match RegisterListBuilder::new()
            .build_type(BuildType::Create)
            .os_id(os_id)
            .build()
        {
            Ok(list) => Some(list),
            Err(e) => match e {
                ShmemError::MappingIdExists => {
                    Some(RegisterListBuilder::new()
                        .build_type(BuildType::Open)
                        .os_id(os_id)
                        .build()
                        .inspect_err(|e| gb_error!("[IpcContext::create_register_list] `RegisterList` open error, create `IpcContext` failed, e = {:?}, os_id = {}", e, os_id))
                        .ok()?)
                }
                _ => {
                    gb_error!(
                        "[IpcContext::create_register_list] `RegisterList` create error, create `IpcContext` failed, e = {:?}, os_id = {}",
                        e, os_id
                    );
                    None
                }
            },
        }
    }

    #[inline]
    pub fn regsiter_terminal(&mut self, info: RegisterInfo) {
        self.terminal_register_list.check_valid();
        self.terminal_register_list.add(info);
    }

    #[inline]
    pub fn regsiter_shell(&mut self, info: RegisterInfo) {
        self.shell_register_list.check_valid();
        self.shell_register_list.add(info);
    }

    #[inline]
    pub fn regsiter_session(&mut self, info: RegisterInfo) {
        self.session_register_list.check_valid();
        self.session_register_list.add(info);
    }

    #[inline]
    pub fn remove_terminal(&mut self, id: RegisterInfoId) {
        self.terminal_register_list.remove(id);
    }

    #[inline]
    pub fn remove_shell(&mut self, id: RegisterInfoId) {
        self.shell_register_list.remove(id);
    }

    #[inline]
    pub fn remove_session(&mut self, id: RegisterInfoId) {
        self.session_register_list.remove(id);
    }

    #[inline]
    pub fn heart_beat_terminal(&mut self, id: RegisterInfoId) {
        if let Some((info, _guard)) = self.terminal_register_list.get_mut(id) {
            info.heart_beat();
        } else {
            gb_warn!(
                "[IpcContext::heart_beat_terminal] Get register info with id `{}` failed.",
                id
            );
        }
    }

    #[inline]
    pub fn heart_beat_shell(&mut self, id: RegisterInfoId) {
        if let Some((info, _guard)) = self.shell_register_list.get_mut(id) {
            info.heart_beat();
        } else {
            gb_warn!(
                "[IpcContext::heart_beat_shell] Get register info with id `{}` failed.",
                id
            );
        }
    }

    #[inline]
    pub fn heart_beat_session(&mut self, id: RegisterInfoId) {
        if let Some((info, _guard)) = self.session_register_list.get_mut(id) {
            info.heart_beat();
        } else {
            gb_warn!(
                "[IpcContext::heart_beat_session] Get register info with id `{}` failed.",
                id
            );
        }
    }

    #[inline]
    pub fn check_register_validation(&mut self) {
        self.terminal_register_list.check_valid();
        self.shell_register_list.check_valid();
        self.session_register_list.check_valid();
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
            IpcRole::Terminal => {
                if let Some(id) = self.queue.try_read() {
                    if let Some((info, _guard)) = self.session_register_list.get_ref(id) {
                        if info.is_alive() { Some(id) } else { None }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => unreachable!(),
        }
    }
}
