use std::sync::Mutex;

use raylib::ffi::Vector2;

pub static EDITOR_STATE: Mutex<EditorState> = Mutex::new(EditorState {
    buffer: vec![],
    commands: vec![],
    input_history: vec![],
});

pub struct EditorState {
    pub buffer: Vec<char>,
    pub commands: Vec<String>,
    pub input_history: Vec<String>,
}

pub static MAP_STATE: Mutex<MapState> = Mutex::new(MapState {
    tiles: vec![],
    player: Player {
        position: Vector2 { x: 0.0, y: 0.0 },
        previous_position: Vector2 { x: 0.0, y: 0.0 }
    },
});

pub struct MapState {
    pub tiles: Vec<Vec<Tile>>,
    pub player: Player,
}

pub struct Player {
    pub position: Vector2,
    pub previous_position: Vector2,
}

#[derive(Debug)]
pub enum MoveError {
    HitWall,
    NoTiles,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl MapState {
    pub fn may_move_player(&mut self, direction: Direction) -> Result<(), MoveError> {
        let current_player_position = self.player.position;
        let (current_x, current_y) = (
            current_player_position.x as usize,
            current_player_position.y as usize,
        );
        let mut next_x = 0;
        let mut next_y = 0;
        match direction {
            Direction::Up => {
                next_x = current_x;
                next_y = current_y - 1;
            }
            Direction::Down => {
                next_x = current_x;
                next_y = current_y + 1;
            }
            Direction::Left => {
                next_x = current_x - 1;
                next_y = current_y;
            }
            Direction::Right => {
                next_x = current_x + 1;
                next_y = current_y;
            }
        };
        println!("{}", next_x);
        let next_tile = if let Some(line) = self.tiles.get(next_y) {
            if let Some(next_tile) = line.get(next_x) {
                next_tile
            } else {
                return Err(MoveError::NoTiles);
            }
        } else {
            return Err(MoveError::NoTiles);
        };
        match next_tile {
            Tile::Wall => return Err(MoveError::HitWall),
            _ => (),
        };
        self.player.position = Vector2 {
            x: next_x as f32,
            y: next_y as f32,
        };
        Ok(())
    }
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
                    map.player.previous_position.x = x as f32;
                    map.player.position.y = y as f32;
                    map.player.previous_position.y = y as f32;
                    is_player_placed = true;
                } else {
                    line.push(Tile::Ground);
                }
            }
        }
        map.tiles.push(line);
    }
}
