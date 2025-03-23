use godot::prelude::*;
use strum_macros::{Display, EnumString, FromRepr};

#[repr(u8)]
#[rustfmt::skip]
#[derive(
    GodotConvert, Var, Export, Display, Debug, FromRepr, EnumString, PartialEq, Eq, Clone, Copy, PartialOrd, Ord
)]
#[godot(via = u8)]
pub enum ShExecuteStatus {
    Done = 0,
    Running = 1,
}

#[derive(GodotClass)]
#[class(init)]
pub struct ExecuteStatus;
#[godot_api]
impl ExecuteStatus {
    #[constant]
    pub const DONE: u8 = ShExecuteStatus::Done as u8;
    #[constant]
    pub const RUNNING: u8 = ShExecuteStatus::Running as u8;
}
