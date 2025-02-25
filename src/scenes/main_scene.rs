use raylib::{
    color::Color,
    ffi::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};

use crate::{
    editor::{editor::{BACKSPACE, CARRIAGE_RETURN, KEYS_PRESSED}, parser::get_prompt_tokens},
    game_state::{EDITOR_STATE, MAP_STATE},
    TILE_SIZE,
};

pub fn main_scene(rl: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32) {
    let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(thread);
    let x_game_anchor: i32 = width / 3;

    d.clear_background(Color::GRAY);

    editor_rendering(&mut d, x_game_anchor, height);

    //Map rendering
    let map = MAP_STATE.lock().expect("Failed to get map state");
    let (mut x, mut y) = (x_game_anchor, 0);
    for line in map.tiles.iter() {
        for tile in line.iter() {
            let color = match tile {
                crate::game_state::Tile::Player => Color::GREEN,
                crate::game_state::Tile::Ground => Color::LIGHTGRAY,
                crate::game_state::Tile::Wall => Color::GRAY,
            };
            d.draw_rectangle_v(
                Vector2 {
                    x: x as f32,
                    y: y as f32,
                },
                Vector2 {
                    x: TILE_SIZE as f32,
                    y: TILE_SIZE as f32,
                },
                color,
            );
            x += 32;
        }
        x = x_game_anchor;
        y += 32;
    }
}

#[allow(static_mut_refs)]
fn editor_rendering(d: &mut RaylibDrawHandle<'_>, x_game_anchor: i32, height: i32) {
    d.draw_rectangle(0, 0, x_game_anchor, height, Color::BLACK);
    let mut editor_state = EDITOR_STATE.lock().expect("Failed to get editor state");

    if let Some(key) = unsafe { KEYS_PRESSED.pop_back() } {
        if key == BACKSPACE {
            editor_state.buffer.pop();
        } else if key == CARRIAGE_RETURN {
            let prompt: String = editor_state.buffer.iter().collect();
            editor_state.buffer = vec![];
            let tokens = get_prompt_tokens(prompt.clone());
            println!("Tokens for the command :");
            tokens.iter().for_each(|token| {
                println!("{:?}", token);
            });
            editor_state.commands.push(prompt.clone());
        } else {
            editor_state.buffer.push(key);
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

