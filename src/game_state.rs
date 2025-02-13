use std::sync::Mutex;

pub static EDITOR_STATE: Mutex<EditorState> = Mutex::new(EditorState { 
    buffer: vec![],
    commands: vec![],
});

pub struct EditorState {
    pub buffer: Vec<char>,
    pub commands: Vec<String>,
}

