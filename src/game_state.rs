use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable, utils::*};

use std::{sync::Mutex, vec};

use raylib::ffi::Vector2;

use crate::{GAME_HEIGHT, GAME_WIDTH};

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
        velocity: 0.25,
        position: Vector2 { x: 0.0, y: 0.0 },
        previous_position: Vector2 { x: 0.0, y: 0.0 },
    },
});

pub struct MapState {
    pub tiles: Vec<Vec<Tile>>,
    pub player: Player,
}

pub struct Player {
    pub velocity: f32,
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
                if current_y == 0 { return Err(MoveError::NoTiles) }
                next_x = current_x;
                next_y = current_y - 1;
            }
            Direction::Down => {
                if current_y == GAME_HEIGHT as usize - 1 { return Err(MoveError::NoTiles) }
                next_x = current_x;
                next_y = current_y + 1;
            }
            Direction::Left => {
                if current_x == 0 { return Err(MoveError::NoTiles) }
                next_x = current_x - 1;
                next_y = current_y;
            }
            Direction::Right => {
                if current_x == GAME_WIDTH as usize -1 { return Err(MoveError::NoTiles) }
                next_x = current_x + 1;
                next_y = current_y;
            }
        };
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
            Tile::Ground | Tile::Water => (),
            _ => return Err(MoveError::HitWall),
        };
        self.player.position = Vector2 {
            x: next_x as f32,
            y: next_y as f32,
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum Tile {
    Ground = 0,
    Wall = 1,
    Water = 2,
    Lava = 3,
    Bronze = 4,
    Silver = 5,
    Gold = 6,
    Mytril = 7,
    Demonite = 8,
    Glitch = 9,
}

pub fn init_map(width: u32, height: u32) {
    let mut map = MAP_STATE.lock().expect("Failed to get map state");
    map.tiles = generate_map(width as usize, height as usize);
    for (y, line) in map.tiles.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if let Tile::Ground = tile {
                map.player.position.x = x as f32;
                map.player.position.y = y as f32;
                map.player.previous_position.x = x as f32;
                map.player.previous_position.y = y as f32;
                return;
            }
        }
    }
}

pub fn generate_map(width: usize, height: usize) -> Vec<Vec<Tile>> {
    println!("GENERATING MAP...");
    let mut tile_map: Vec<Vec<Tile>> = vec![];
    let hasher = PermutationTable::new(0);
    let map: NoiseMap = PlaneMapBuilder::new_fn(|point| perlin_2d(point.into(), &hasher))
        .set_size(width, height)
        .set_x_bounds(-40.0, 40.0)
        .set_y_bounds(-40.0, 40.0)
        .build();
    let mut x = 0;
    let mut row = vec![];
    for cell in map {
        row.push(to_tile(cell));
        x += 1;
        if x >= width {
            x = 0;
            tile_map.push(row);
            row = vec![];
        }
    }
    tile_map
}

fn to_tile(cell: f64) -> Tile {
    if cell < 0.0 {
        Tile::Wall
    } else {
        Tile::Ground
    }
}
