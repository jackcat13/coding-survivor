use std::sync::Mutex;

use editor::editor::start_keyboard_thread;
use game_state::init_map;
use raylib::{RaylibHandle, RaylibThread};
use scenes::main_scene::main_scene;

mod editor;
mod game_state;
mod scenes;

const GAME_NAME: &str = "Coding Survivor";
const TARGET_FPS: u32 = 60;
const GAME_WIDTH: u32 = 1000;
const GAME_HEIGHT: u32 = 1000;
const TILE_SIZE: u8 = 32;

const GET_EDITOR_STATE_ERROR: &str = "Failed to get editor state";

type SceneFnPointer = fn(&mut RaylibHandle, &RaylibThread, i32, i32);
static CURRENT_SCENE: Mutex<SceneFnPointer> = Mutex::new(main_scene);

fn main() {
    init_map(GAME_WIDTH, GAME_HEIGHT);

    let (mut rl, thread) = raylib::init()
        .size(0, 0)
        .title(GAME_NAME)
        .build();

    rl.set_target_fps(TARGET_FPS);

    start_keyboard_thread();

    while !rl.window_should_close() {
        let current_scene = CURRENT_SCENE.lock().expect("Failed to get current scene");
        let (game_width, game_height) = (rl.get_screen_width(), rl.get_screen_height());
        current_scene(&mut rl, &thread, game_width, game_height);
    }
}


