use std::collections::HashMap;

use raylib::{
    camera::Camera2D,
    color::Color,
    ffi::{BlendMode, Rectangle},
    math::Vector2,
    prelude::{RaylibBlendModeExt, RaylibDraw, RaylibDrawHandle, RaylibMode2DExt},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

use crate::{
    animation::Animation, editor::functions::FUNCTIONS, game_state::{get_tile_string, EDITOR_STATE, MAP_STATE}, GAME_HEIGHT, GAME_WIDTH, GET_EDITOR_STATE_ERROR, TILE_SIZE
};

pub fn main_scene(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    width: i32,
    height: i32,
    map_textures: &HashMap<String, Texture2D>,
    player_animation: &Animation,
) {
    let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(thread);
    let x_game_anchor: i32 = width / 3;

    d.clear_background(Color::BLACK);

    process_player_position();
    map_rendering(
        &mut d,
        x_game_anchor,
        width,
        height,
        map_textures,
        player_animation,
    );
    editor_rendering(&mut d, x_game_anchor, height, x_game_anchor);
    d.draw_text(
        &format!("FPS : {}", d.get_fps()),
        width - 100,
        0,
        20,
        Color::WHITE,
    );
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
const EDITOR_COLOR: Color = Color::WHITE;

#[allow(static_mut_refs)]
fn editor_rendering(d: &mut RaylibDrawHandle<'_>, x_game_anchor: i32, height: i32, width: i32) {
    let editor_state = EDITOR_STATE.lock().expect(GET_EDITOR_STATE_ERROR);
    d.draw_rectangle(0, 0, x_game_anchor, height, Color::WHITE.alpha(0.05));
    d.draw_line(x_game_anchor, 0, width, height, Color::DARKGOLDENROD);
    let input_line: String = editor_state.buffer.iter().collect();
    let input_line = "> ".to_owned() + &input_line;
    d.draw_text(
        &input_line,
        EDITOR_TEXT_X,
        EDITOR_PROMPT_Y,
        EDITOR_FONT_SIZE,
        EDITOR_COLOR,
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
    if let Some(current_token) = get_current_token(&input_line) {
        let completions = get_completions(&current_token);
        let mut y_completion = EDITOR_PROMPT_Y + 30;
        for completion in completions.iter() {
            d.draw_rectangle(EDITOR_TEXT_X, y_completion, width, 30, Color::DARKSLATEGRAY);
            d.draw_text(completion, EDITOR_TEXT_X + 5, y_completion + 5, EDITOR_FONT_SIZE, EDITOR_COLOR);
            y_completion += 30;
        }
    }
}

fn get_completions(current_token: &str) -> Vec<String> {
    let functions = FUNCTIONS.lock().expect("Failed to resolve functions");
    functions
        .iter()
        .filter(|function| function.is_matching(current_token.to_string()))
        .map(|function| function.to_complete_string())
        .collect()
}

fn get_current_token(input_line: &str) -> Option<String> {
    if input_line.is_empty() {
        return None;
    }
    let mut result = String::new();
    for character in input_line.chars().rev() {
        if character != ' ' {
            result.push(character);
        } else {
            break;
        }
    }
    if result.is_empty() {
        return None;
    }
    Some(result.chars().rev().collect())
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

const TEXTURE_ERROR: &str = "Failed to resolve c_string for textures";
const BOUND_ERROR: &str = "Failed to resolve boundaries";

fn map_rendering(
    d: &mut RaylibDrawHandle,
    x_game_anchor: i32,
    width: i32,
    height: i32,
    textures: &HashMap<String, Texture2D>,
    player_animation: &Animation,
) {
    let mut map = MAP_STATE.lock().expect("Failed to get map state");
    let player_x = map.player.previous_position.x * TILE_SIZE as f32;
    let player_y = map.player.previous_position.y * TILE_SIZE as f32;
    let camera = Camera2D {
        offset: Vector2 {
            x: (width as f32 + x_game_anchor as f32) / 2.0,
            y: height as f32 / 2.0,
        },
        target: Vector2 {
            x: player_x,
            y: player_y,
        },
        rotation: f32::default(),
        zoom: map.zoom,
    };
    let mut d = d.begin_mode2D(camera);
    // Light source
    let render_distance = TILE_SIZE as f32 * map.player.light_vision;
    d.draw_circle(player_x as i32 + TILE_SIZE as i32 / 2, player_y as i32 + TILE_SIZE as i32 / 2, render_distance, Color::WHITE);
    let mut d = d.begin_blend_mode(BlendMode::BLEND_MULTIPLIED);
    // Map rendering
    let (range_x, range_y, x_start, y_start) = get_map_rendering_bounds(map.player.position.x, map.player.position.y);
    let x_start = x_start as i32 * TILE_SIZE as i32;
    let (mut x, mut y) = (x_start, y_start as i32 * TILE_SIZE as i32);
    for line in map.tiles[range_y.clone()].iter() {
        for tile in line[range_x.clone()].iter() {
            d.draw_texture(&textures[&get_tile_string(tile)], x, y, Color::WHITE);
            x += TILE_SIZE as i32;
        }
        x = x_start;
        y += TILE_SIZE as i32;
    }
    // Items rendering
    let mut d = d.begin_blend_mode(BlendMode::BLEND_ADDITIVE);
    for item in &map.items {
        let item_x = item.position.x as i32 * TILE_SIZE as i32;
        let item_y = item.position.y as i32 * TILE_SIZE as i32;
        d.draw_texture(&textures[&item.item.get_name()], item_x, item_y, Color::WHITE);
    }
    // Player rendering
    let mut d = d.begin_blend_mode(BlendMode::BLEND_ALPHA);
    d.draw_texture_rec(
        &player_animation.texture,
        Rectangle {
            x: player_animation.origin.x
                + (map.player.animation_state.current_frame * player_animation.frame_width) as f32,
            y: player_animation.origin.y,
            width: TILE_SIZE as f32,
            height: TILE_SIZE as f32,
        },
        Vector2 {
            x: player_x,
            y: player_y,
        },
        Color::WHITE,
    );
    map.player
        .animation_state
        .next_frame(player_animation.frame_number);
}

const MAP_MAX_RENDER_DISTANCE: f32 = 50.0;
fn get_map_rendering_bounds(
    player_x: f32,
    player_y: f32,
) -> (std::ops::Range<usize>, std::ops::Range<usize>, usize, usize) {
    let min_x = if player_x - MAP_MAX_RENDER_DISTANCE < 0.0 {
        0.0
    } else {
        player_x - MAP_MAX_RENDER_DISTANCE
    };
    let max_x = if player_x + MAP_MAX_RENDER_DISTANCE > GAME_WIDTH as f32 - 1.0 {
        GAME_WIDTH as f32 - 1.0
    } else {
        player_x + MAP_MAX_RENDER_DISTANCE
    };
    let min_y = if player_y - MAP_MAX_RENDER_DISTANCE < 0.0 {
        0.0
    } else {
        player_y - MAP_MAX_RENDER_DISTANCE
    };
    let max_y = if player_y + MAP_MAX_RENDER_DISTANCE > GAME_HEIGHT as f32 - 1.0 {
        GAME_HEIGHT as f32 - 1.0
    } else {
        player_y + MAP_MAX_RENDER_DISTANCE
    };
    (
        min_x as usize..max_x as usize,
        min_y as usize..max_y as usize,
        min_x as usize,
        min_y as usize,
    )
}
