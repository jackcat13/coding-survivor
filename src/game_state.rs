use std::sync::Mutex;

use raylib::ffi::Vector2;

pub static EDITOR_STATE: Mutex<EditorState> = Mutex::new(EditorState { 
    buffer: vec![],
    commands: vec![],
});

pub struct EditorState {
    pub buffer: Vec<char>,
    pub commands: Vec<String>,
}

pub static MAP_STATE: Mutex<MapState> = Mutex::new(MapState {
    tiles: vec![],
    player_position: Vector2{ x: 0.0, y: 0.0 },
});

pub struct MapState {
    pub tiles: Vec<Vec<Tile>>,
    pub player_position: Vector2,
}

pub enum Tile {
    Ground, Wall,
}

pub fn init_map(width: u32, height: u32) {
    let mut map = MAP_STATE.lock().expect("Failed to get map state");
    for y in 0..height {
        let mut line = vec![];
        for x in 0..width {
            if x % 2 == 0 && y % 2 == 0 {
                line.push(Tile::Wall);
            } else {
                line.push(Tile::Ground);
            }
        }
        map.tiles.push(line);
    }
}
