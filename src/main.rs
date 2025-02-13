use std::sync::Mutex;

use raylib::{color::Color, prelude::RaylibDraw, RaylibHandle, RaylibThread};

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
    let mut d = rl.begin_drawing(thread);
    
    d.clear_background(Color::GRAY);
    d.draw_rectangle(0, 0, width / 3, height, Color::BLACK);
}

