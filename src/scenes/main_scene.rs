use raylib::{
    color::Color,
    ffi::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};

use crate::{
    game_state::{EDITOR_STATE, MAP_STATE},
    GET_EDITOR_STATE_ERROR, TILE_SIZE,
};

pub fn main_scene(rl: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32) {
    let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(thread);
    let x_game_anchor: i32 = width / 3;

    d.clear_background(Color::GRAY);

    process_player_position();
    editor_rendering(&mut d, x_game_anchor, height, x_game_anchor);
    map_rendering(&mut d, x_game_anchor);
}

fn process_player_position() {
    let mut map = MAP_STATE.lock().expect("Failed to get map state");
    if map.player.previous_position.x < map.player.position.x {
        map.player.previous_position.x += map.player.velocity;
    } else if map.player.previous_position.x > map.player.position.x {
        map.player.previous_position.x -= map.player.velocity;
    }
    if map.player.previous_position.y < map.player.position.y {
        map.player.previous_position.y += map.player.velocity;
    } else if map.player.previous_position.y > map.player.position.y {
        map.player.previous_position.y -= map.player.velocity;
    }
}

const EDITOR_PROMPT_Y: i32 = 10;
const EDITOR_HISTORY_Y: i32 = 40;
const EDITOR_HISTORY_LINE_HEIGHT: i32 = 30;
const EDITOR_TEXT_X: i32 = 20;
const EDITOR_FONT_SIZE: i32 = 20;

#[allow(static_mut_refs)]
fn editor_rendering(d: &mut RaylibDrawHandle<'_>, x_game_anchor: i32, height: i32, width: i32) {
    let editor_state = EDITOR_STATE.lock().expect(GET_EDITOR_STATE_ERROR);
    d.draw_rectangle(0, 0, x_game_anchor, height, Color::BLACK);
    let input_line: String = editor_state.buffer.iter().collect();
    let input_line = "> ".to_owned() + &input_line;
    d.draw_text(
        &input_line,
        EDITOR_TEXT_X,
        EDITOR_PROMPT_Y,
        EDITOR_FONT_SIZE,
        Color::WHITE,
    );
    let mut y_history_position = EDITOR_HISTORY_Y;
    for history_text in editor_state.commands.iter().rev() {
        if y_history_position > height {
            break;
        }
        let (text, color) = resolve_history_text_format(history_text.to_string());
        let character_width = EDITOR_FONT_SIZE as f32 / 1.5;
        let lines = ((text.len() as i32 * character_width as i32) / width) + 1;
        let mut x_index = 0;
        let line_max_width = width / character_width as i32;
        for _ in 0..lines {
            let mut x_index_end = x_index + line_max_width;
            if x_index_end > text.len() as i32 {
                x_index_end = text.len() as i32
            }
            let text_slice = &text[x_index as usize..x_index_end as usize];
            d.draw_text(
                text_slice,
                EDITOR_TEXT_X,
                y_history_position,
                EDITOR_FONT_SIZE,
                color,
            );
            x_index += line_max_width;
            y_history_position += EDITOR_HISTORY_LINE_HEIGHT;
        }
    }
}

fn map_rendering(d: &mut RaylibDrawHandle, x_game_anchor: i32) {
    let map = MAP_STATE.lock().expect("Failed to get map state");
    let (mut x, mut y) = (x_game_anchor, 0);
    for line in map.tiles.iter() {
        for tile in line.iter() {
            let color = match tile {
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
    let player_x = map.player.previous_position.x * TILE_SIZE as f32 + x_game_anchor as f32;
    let player_y = map.player.previous_position.y * TILE_SIZE as f32;
    d.draw_rectangle_v(
        Vector2 {
            x: player_x,
            y: player_y,
        },
        Vector2 {
            x: TILE_SIZE as f32,
            y: TILE_SIZE as f32,
        },
        Color::GREEN,
    );
}

fn resolve_history_text_format(history_text: String) -> (String, Color) {
    if history_text.starts_with("ERR-") {
        let text = history_text.replace("ERR-", "");
        return (text, Color::RED);
    } else if history_text.starts_with("RES-") {
        let text = history_text.replace("RES-", "");
        return (text, Color::GREEN);
    }
    (history_text, Color::WHITE)
}
