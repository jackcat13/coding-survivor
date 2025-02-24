use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::game_state::EDITOR_STATE;

pub fn process_input(rl: &mut RaylibHandle) {
    loop {
        let char_pressed = rl.get_char_pressed();
        let key_pressed = rl.get_key_pressed();
     
        let mut editor_state = EDITOR_STATE.lock().expect("Failed to get editor state");
        if let Some(input) = char_pressed {
            editor_state.buffer.push(input);
        }
        if let Some(key) = key_pressed {
            if key == KeyboardKey::KEY_BACKSPACE {
                editor_state.buffer.pop();
            } else if key == KeyboardKey::KEY_ENTER {
                let command: String = editor_state.buffer.iter().collect();
                editor_state.buffer = vec![];
                editor_state.commands.push(command.clone());
            }
        }
    }
}
