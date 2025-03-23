use godot::builtin::{Array, GString};

pub type InternalCommand = Box<dyn IInternalCommand>;

pub trait IInternalCommand {
    fn command_name(&self) -> String;

    fn start(&mut self, params: Array<GString>);
}
