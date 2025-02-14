use std::sync::Mutex;

use raylib::{RaylibHandle, RaylibThread};
use scenes::main_scene::main_scene;

mod game_state;
mod scenes;

const GAME_NAME: &str = "Coding Survivor";
type SceneFnPointer = fn(&mut RaylibHandle, &RaylibThread, i32, i32);
static CURRENT_SCENE: Mutex<SceneFnPointer> = Mutex::new(main_scene);

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


