use termio::cli::{constant::ProtocolType, session::SessionPropsId};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Session {
    pub(crate) id: SessionPropsId,
    pub(crate) ty: ProtocolType,
}

impl Session {
    #[inline]
    pub fn new(id: SessionPropsId, ty: ProtocolType) -> Self {
        Self { id, ty }
    }
}
