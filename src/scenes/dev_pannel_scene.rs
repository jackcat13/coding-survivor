use std::collections::HashMap;

use ollama_rs::Ollama;
use raylib::{
    ffi::Rectangle, prelude::RaylibDrawHandle, rgui::RaylibDrawGui, rstr, texture::Texture2D, RaylibHandle, RaylibThread
};

use crate::{game_state::{generate_map, Tile}, GAME_HEIGHT, GAME_WIDTH};

const AI_MODEL: &str = "llama3:latest";
const MAP_GENERATOR_BUTTON_X: f32 = 50.0;
const MAP_GENERATOR_BUTTON_Y: f32 = 50.0;
const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 30.0;

pub fn dev_pannel_scene(rl: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32, _textures: &HashMap<Tile, Texture2D>) {
    let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(thread);
    if d.gui_button(
        Rectangle {
            x: MAP_GENERATOR_BUTTON_X,
            y: MAP_GENERATOR_BUTTON_Y,
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
        },
        Some(rstr!("Generate Map")),
    ) {
         println!("{:?}", generate_map(100, 100));
    }
}


struct AiClient {
    ollama: Ollama,
    model: String,
}

impl AiClient {
    fn generate_map(&self) -> Result<String, String> {
        Err("Failed to generate map".to_string())
    }
}
