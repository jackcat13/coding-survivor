use std::{collections::HashMap, sync::Mutex};

use animation::Animation;
use editor::keyboard::{editor_processing, start_keyboard_thread};
use game_state::{init_map, Tile};
use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};
use scenes::main_scene::main_scene;
use textures::{load_map_texture, load_player_animation};

mod animation;
mod editor;
mod game_state;
mod scenes;
mod textures;

const GAME_NAME: &str = "Coding Survivor";
const TARGET_FPS: u32 = 60;
const GAME_WIDTH: u32 = 1000;
const GAME_HEIGHT: u32 = 1000;
const TILE_SIZE: u8 = 32;

const GET_EDITOR_STATE_ERROR: &str = "Failed to get editor state";

type SceneFnPointer = fn(&mut RaylibHandle, &RaylibThread, i32, i32, &HashMap<Tile, Texture2D>, &Animation);
static CURRENT_SCENE: Mutex<SceneFnPointer> = Mutex::new(main_scene);

fn main() {
    init_map(GAME_WIDTH, GAME_HEIGHT);

    #[cfg(feature = "dev-only")]
    use_dev_pannel();

    let (mut rl, thread) = raylib::init()
        .size(0, 0)
        .title(GAME_NAME)
        .resizable()
        .build();

    rl.set_target_fps(TARGET_FPS);

    start_keyboard_thread();
    editor_processing();
    let map_textures = load_map_texture(&mut rl, &thread);
    let player_animation = load_player_animation(&mut rl, &thread);

    while !rl.window_should_close() {
        let current_scene = CURRENT_SCENE.lock().expect("Failed to get current scene");
        let (game_width, game_height) = (rl.get_screen_width(), rl.get_screen_height());
        current_scene(&mut rl, &thread, game_width, game_height, &map_textures, &player_animation);
    }
}

#[cfg(feature = "dev-only")]
fn use_dev_pannel() {
    use scenes::dev_pannel_scene::dev_pannel_scene;

    let mut current_scene = CURRENT_SCENE.lock().expect("Failed to get current scene");
    *current_scene = dev_pannel_scene;
}
