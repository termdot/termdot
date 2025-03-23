pub mod execute_status;
pub mod internal;

use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Command {
    #[export]
    command_name: GString,
    base: Base<Node>,
}

#[allow(unused_variables)]
#[godot_api]
impl Command {
    #[func(virtual, gd_self)]
    pub fn start(gd: Gd<Self>, params: Array<GString>) {}
}
