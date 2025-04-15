use godot::{
    classes::{
        EditorInterface, EditorPlugin, IEditorPlugin, InputEvent, InputEventKey, Label,
        RichTextLabel, TextEdit,
    },
    global::Key,
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = EditorPlugin, tool)]
pub struct TermdotPlugin {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for TermdotPlugin {
    fn ready(&mut self) {
        let script_editor = match EditorInterface::singleton().get_script_editor() {
            Some(editor) => editor,
            None => {
                return;
            }
        };
        let cur_editor = match script_editor.get_current_editor() {
            Some(seb) => seb,
            None => {
                return;
            }
        };

        let mut nodes = cur_editor.get_children_ex().include_internal(true).done();
        while let Some(node) = nodes.pop_front() {
            if let Ok(_label) = node.clone().try_cast::<Label>() {}
            if let Ok(_text) = node.clone().try_cast::<RichTextLabel>() {}
            // Where storage the script codes
            if let Ok(_text_edit) = node.clone().try_cast::<TextEdit>() {}

            for c in node
                .get_children_ex()
                .include_internal(true)
                .done()
                .iter_shared()
            {
                nodes.push(&c);
            }
        }
    }

    fn enter_tree(&mut self) {}

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(key_event) = event.try_cast::<InputEventKey>() {
            if key_event.is_pressed() && key_event.get_keycode() == Key::F12 {
            }
        }
    }
}
