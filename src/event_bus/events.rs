#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EventType {
    MasterExit = 0,
    MasterReady,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Events {
    MasterExit,
    MasterReay,
}
impl Events {
    #[inline]
    pub fn ty(&self) -> EventType {
        match self {
            Self::MasterExit => EventType::MasterExit,
            Self::MasterReay => EventType::MasterReady,
        }
    }
}
