pub mod command;
pub mod termdot;

use godot::prelude::*;

struct TermdotBuiltin;

#[gdextension]
unsafe impl ExtensionLibrary for TermdotBuiltin {}
