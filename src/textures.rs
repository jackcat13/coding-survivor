use std::collections::HashMap;

use raylib::{ffi::Vector2, texture::Texture2D, RaylibHandle, RaylibThread};

use crate::{animation::Animation, game_state::{Tile, MAP_STATE}};

pub fn load_map_texture(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<Tile, Texture2D> {
    let mut textures = HashMap::new();
    textures.insert(
        Tile::Ground,
        rl.load_texture(thread, "assets/map/ground.png")
            .expect("Failed to load ground texture"),
    );
    textures.insert(
        Tile::Wall,
        rl.load_texture(thread, "assets/map/wall.png")
            .expect("Failed to load wall texture"),
    );
    textures.insert(
        Tile::Lava,
        rl.load_texture(thread, "assets/map/lava.png")
            .expect("Failed to load lava texture"),
    );
    textures.insert(
        Tile::Gold,
        rl.load_texture(thread, "assets/map/gold.png")
            .expect("Failed to load gold texture"),
    );
    textures.insert(
        Tile::Water,
        rl.load_texture(thread, "assets/map/water.png")
            .expect("Failed to load water texture"),
    );
    textures.insert(
        Tile::Bronze,
        rl.load_texture(thread, "assets/map/bronze.png")
            .expect("Failed to load bronze texture"),
    );
    textures.insert(
        Tile::Silver,
        rl.load_texture(thread, "assets/map/silver.png")
            .expect("Failed to load silver texture"),
    );
    textures.insert(
        Tile::Mytril,
        rl.load_texture(thread, "assets/map/mytril.png")
            .expect("Failed to load mytril texture"),
    );
    textures.insert(
        Tile::Glitch,
        rl.load_texture(thread, "assets/map/glitch.png")
            .expect("Failed to load glitch texture"),
    );
    textures.insert(
        Tile::Demonite,
        rl.load_texture(thread, "assets/map/demonite.png")
            .expect("Failed to load demonite texture"),
    );
    textures
}

pub fn load_player_animation(rl: &mut RaylibHandle, thread: &RaylibThread) -> Animation {
    let state = MAP_STATE.lock().expect("Failed to load map state");
    let texture_path = match state.player.animation_state.status {
        crate::game_state::Status::Idle => "assets/player/playerIdle.png",
    };
    let texture = rl
        .load_texture(thread, texture_path)
        .expect("Failed to load demonite texture");
    Animation {
        origin: Vector2{
            x: 32.0,
            y: 33.0,
        },
        frame_number: 6,
        frame_width: 100,
        texture,
    }
}
