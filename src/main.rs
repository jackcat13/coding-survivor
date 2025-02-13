use std::sync::Mutex;

use game_state::EDITOR_STATE;
use raylib::{color::Color, ffi::KeyboardKey, prelude::RaylibDraw, RaylibHandle, RaylibThread};

mod game_state;

const GAME_NAME: &str = "Coding Survivor";
static CURRENT_SCENE: Mutex<fn(&mut RaylibHandle, &RaylibThread, i32, i32)> = Mutex::new(main_scene);

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(0, 0)
        .title(GAME_NAME)
        .build();


    while !rl.window_should_close() {
        let current_scene = CURRENT_SCENE.lock().expect("Failed to get current scene");
        let (game_width, game_height) = (rl.get_screen_width(), rl.get_screen_height());
        current_scene(&mut rl, &thread, game_width, game_height);
    }
}

fn main_scene(rl: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32) {
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
        }
    }
    let input_line: String = editor_state.buffer.iter().collect();
    let input_line = "> ".to_owned() + &input_line;
    d.draw_text(&input_line, 20, 10, 20, Color::WHITE);
}

