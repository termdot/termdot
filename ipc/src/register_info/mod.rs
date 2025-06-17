pub mod register_list;

pub const DEAD_TIME_MILLIS: u128 = 500;

use common::typedef::RegisterInfoId;
use std::time::Instant;

pub trait IRegisterInfo: 'static + Copy {
    fn id(&self) -> RegisterInfoId;

    fn is_valid(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterInfo {
    register_id: RegisterInfoId,
    heart_beat: Instant,
}

impl RegisterInfo {
    #[inline]
    pub fn new(register_id: RegisterInfoId) -> Self {
        Self {
            register_id,
            heart_beat: Instant::now(),
        }
    }

    #[inline]
    pub fn heart_beat(&mut self) {
        self.heart_beat = Instant::now();
    }

    #[inline]
    pub fn is_alive(&self) -> bool {
        let duration = Instant::now() - self.heart_beat;
        duration.as_millis() < DEAD_TIME_MILLIS
    }
}

impl IRegisterInfo for RegisterInfo {
    #[inline]
    fn id(&self) -> RegisterInfoId {
        self.register_id
    }

    #[inline]
    fn is_valid(&self) -> bool {
        self.is_alive()
    }
}
