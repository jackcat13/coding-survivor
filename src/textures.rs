use std::collections::HashMap;

use raylib::{ffi::Vector2, texture::Texture2D, RaylibHandle, RaylibThread};

use crate::{
    animation::Animation,
    game_state::{get_tile_string, Status, Tile, MAP_STATE},
};

pub fn load_map_texture(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
) -> HashMap<String, Texture2D> {
    let mut textures = HashMap::new();
    textures.insert(
        get_tile_string(&Tile::Ground),
        rl.load_texture(thread, "assets/map/ground.png")
            .expect("Failed to load ground texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Wall),
        rl.load_texture(thread, "assets/map/wall.png")
            .expect("Failed to load wall texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Lava),
        rl.load_texture(thread, "assets/map/lava.png")
            .expect("Failed to load lava texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Gold),
        rl.load_texture(thread, "assets/map/gold.png")
            .expect("Failed to load gold texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Water),
        rl.load_texture(thread, "assets/map/water.png")
            .expect("Failed to load water texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Bronze),
        rl.load_texture(thread, "assets/map/bronze.png")
            .expect("Failed to load bronze texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Silver),
        rl.load_texture(thread, "assets/map/silver.png")
            .expect("Failed to load silver texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Mytril),
        rl.load_texture(thread, "assets/map/mytril.png")
            .expect("Failed to load mytril texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Glitch),
        rl.load_texture(thread, "assets/map/glitch.png")
            .expect("Failed to load glitch texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Demonite),
        rl.load_texture(thread, "assets/map/demonite.png")
            .expect("Failed to load demonite texture"),
    );
    textures.insert(
        get_tile_string(&Tile::Tree),
        rl.load_texture(thread, "assets/map/tree.png")
            .expect("Failed to load tree texture"),
    );
    textures.insert(
        "Pickaxe".to_string(),
        rl.load_texture(thread, "assets/items/pickaxe.png")
            .expect("Failed to load pickaxe texture"),
    );
    textures.insert(
        "TreeItem".to_string(),
        rl.load_texture(thread, "assets/items/treeItem.png")
            .expect("Failed to load tree item texture"),
    );
    textures
}

pub fn load_player_animations(rl: &mut RaylibHandle, thread: &RaylibThread) -> Vec<Animation> {
    vec![
        "assets/player/playerIdle.png",
        "assets/player/playerBreaking.png",
    ]
    .iter()
    .map(|animation_path| {
        let texture = rl
            .load_texture(thread, animation_path)
            .expect("Failed to load animation texture");
        Animation {
            origin: Vector2 { x: 32.0, y: 33.0 },
            frame_number: 6,
            frame_width: 100,
            texture,
        }
    })
    .collect()
}

pub fn resolve_animation_index(status: Status) -> usize {
    match status {
        Status::Idle => 0,
        Status::Breaking => 1,
    }
}
