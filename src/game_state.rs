use std::sync::Mutex;

pub static EDITOR_STATE: Mutex<EditorState> = Mutex::new(EditorState { buffer: vec![] });

pub struct EditorState {
    pub buffer: Vec<char>,
}

