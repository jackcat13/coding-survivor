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
    player: Player {
        position: Vector2 { x: 0.0, y: 0.0 },
    },
});

pub struct MapState {
    pub tiles: Vec<Vec<Tile>>,
    pub player: Player,
}

#[derive(Debug)]
pub enum MoveError {
    HitWall, NoTiles,
}

impl MapState {
    pub fn may_move_player_down(&mut self) -> Result<(), MoveError> {
        let current_player_position = self.player.position;
        let (current_x, current_y) = (
            current_player_position.x as usize,
            current_player_position.y as usize,
        );
        let next_y = current_y + 1;
        if let Some(line) = self.tiles.get(next_y) {
            if let Some(next_tile) = line.get(current_x) {
                match next_tile {
                    Tile::Wall => return Err(MoveError::HitWall),
                    _ => (),
                };
                self.player.position = Vector2 {
                    x: current_x as f32,
                    y: next_y as f32,
                };
                return Ok(());
            }
        }
        Err(MoveError::NoTiles)
    }
}

pub struct Player {
    pub position: Vector2,
}

pub enum Tile {
    Ground,
    Wall,
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
                    map.player.position.x = x as f32;
                    map.player.position.y = y as f32;
                    is_player_placed = true;
                } else {
                    line.push(Tile::Ground);
                }
            }
        }
        map.tiles.push(line);
    }
}
