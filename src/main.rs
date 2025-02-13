use raylib::{color::Color, prelude::RaylibDraw, RaylibHandle, RaylibThread};

const GAME_NAME: &str = "Coding Survivor";

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(0, 0)
        .title(GAME_NAME)
        .build();

    while !rl.window_should_close() {
        main_scene(&mut rl, &thread);
    }
}

fn main_scene(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::BLACK);
    d.draw_text("Hello World", 12, 12, 20, Color::WHITE);
}

