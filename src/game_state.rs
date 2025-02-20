use std::sync::Mutex;

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
});

pub struct MapState {
    pub tiles: Vec<Vec<Tile>>,
}

pub enum Tile {
    Ground, Wall, Player,
}

pub fn init_map(width: u32, height: u32) {
    let mut map = MAP_STATE.lock().expect("Failed to get map state");
    let mut is_player_placed = false;
    for y in 0..height {
        let mut line = vec![];
        for x in 0..width {
            if x % 2 == 0 && y % 2 == 0 {
                line.push(Tile::Wall);
            } else {
                if !is_player_placed {
                    line.push(Tile::Player);
                    is_player_placed = true;
                } else {
                    line.push(Tile::Ground);
                }
            }
        }
        map.tiles.push(line);
    }
}
