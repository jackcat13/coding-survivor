use raylib::{color::Color, ffi::KeyboardKey, prelude::RaylibDraw, RaylibHandle, RaylibThread};

use crate::game_state::EDITOR_STATE;

pub fn main_scene(rl: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32) {
    let char_pressed = rl.get_char_pressed();
    let key_pressed = rl.get_key_pressed();
    let mut d = rl.begin_drawing(thread);
    
    d.clear_background(Color::GRAY);
    d.draw_rectangle(0, 0, width / 3, height, Color::BLACK);
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
    let input_line: String = editor_state.buffer.iter().collect();
    let input_line = "> ".to_owned() + &input_line;
    d.draw_text(&input_line, 20, 10, 20, Color::WHITE);
    let mut y_history_position = 40;
    for history_text in editor_state.commands.iter().rev() {
        d.draw_text(history_text, 20, y_history_position, 20, Color::WHITE);
        y_history_position += 30;
    }
}
