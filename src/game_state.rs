use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable, utils::*};

use std::{
    ops::{Deref, DerefMut}, sync::Mutex, time::SystemTime
};

use rand::prelude::Rng;
use raylib::ffi::Vector2;

use crate::{item::{Item, MapItem, Pickaxe}, GAME_HEIGHT, GAME_WIDTH};

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
        animation_state: AnimationState {
            current_frame: 0,
            status: Status::Idle,
        },
        light_vision: 7.0,
    },
    zoom: 1.4,
    items: vec![],
});

pub struct MapState {
    pub tiles: Vec<Vec<Tile>>,
    pub player: Player,
    pub zoom: f32,
    pub items: Vec<MapItem>,
}

pub struct Player {
    pub velocity: f32,
    pub position: Vector2,
    pub previous_position: Vector2,
    pub animation_state: AnimationState,
    pub light_vision: f32,
}

pub struct AnimationState {
    pub current_frame: u32,
    pub status: Status,
}

impl AnimationState {
    pub(crate) fn next_frame(&mut self, frame_number: u32) {
        if self.current_frame < frame_number {
            self.current_frame += 1;
        } else {
            self.current_frame = 0;
        }
    }
}

pub enum Status {
    Idle,
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
                if current_y == 0 {
                    return Err(MoveError::NoTiles);
                }
                next_x = current_x;
                next_y = current_y - 1;
            }
            Direction::Down => {
                if current_y == GAME_HEIGHT as usize - 1 {
                    return Err(MoveError::NoTiles);
                }
                next_x = current_x;
                next_y = current_y + 1;
            }
            Direction::Left => {
                if current_x == 0 {
                    return Err(MoveError::NoTiles);
                }
                next_x = current_x - 1;
                next_y = current_y;
            }
            Direction::Right => {
                if current_x == GAME_WIDTH as usize - 1 {
                    return Err(MoveError::NoTiles);
                }
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


    pub fn spawn_item(&mut self, position: &Vector2, item: Box<dyn Item>) {
        self.items.push(MapItem{
            position: *position,
            item,
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

pub fn get_tile_string(tile: &Tile) -> String {
    match tile {
        Tile::Ground => "Ground".to_string(),
        Tile::Wall => "Wall".to_string(),
        Tile::Water => "Water".to_string(),
        Tile::Lava => "Lava".to_string(),
        Tile::Bronze => "Bronze".to_string(),
        Tile::Silver => "Silver".to_string(),
        Tile::Gold => "Gold".to_string(),
        Tile::Mytril => "Mytril".to_string(),
        Tile::Demonite => "Demonite".to_string(),
        Tile::Glitch => "Glitch".to_string(),
    }
}

impl Deref for Tile {
    type Target = Tile;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

pub fn init_map(width: u32, height: u32) {
    let mut map = MAP_STATE.lock().expect("Failed to get map state");
    map.tiles = generate_map(width as usize, height as usize);
    for (y, line) in map.tiles.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if x >= (line.len() / 2) && y >= (map.tiles.len() / 2) {
                if let Tile::Ground = tile {
                    map.player.position.x = x as f32;
                    map.player.position.y = y as f32;
                    map.player.previous_position.x = x as f32;
                    map.player.previous_position.y = y as f32;
                    map.spawn_item(&Vector2 { x: (x + 1) as f32, y: y as f32 }, Box::new(Pickaxe{}));
                    return;
                }
            }
        }
    }
}

pub fn generate_map(width: usize, height: usize) -> Vec<Vec<Tile>> {
    let now = SystemTime::now();
    println!("GENERATING MAP...");
    let mut rng = rand::rng();
    let seed = rng.random_range(0..u32::MAX);
    let hasher = PermutationTable::new(seed);
    let map: NoiseMap = PlaneMapBuilder::new_fn(|point| perlin_2d(point.into(), &hasher))
        .set_size(width, height)
        .set_x_bounds(-50.0, 50.0)
        .set_y_bounds(-50.0, 50.0)
        .build();
    let map: Vec<f64> = map.iter().copied().collect();
    let rows: Vec<Vec<Tile>> = (0..height as i32)
        .map(|line| {
            let index = line as usize * width;
            let range = index..index + width - 1;
            let row: Vec<Tile> = map[range]
                .iter()
                .map(|noise_value| to_tile(*noise_value))
                .collect();
            row
        })
        .collect();
    if let Ok(elapsed) = now.elapsed() {
        println!(
            "{} * {} map generated in {} ms",
            width,
            height,
            elapsed.as_millis()
        )
    }
    rows
}

fn to_tile(cell: f64) -> Tile {
    if cell < 0.0 {
        to_wall()
    } else if cell > 0.5 && cell < 0.505 {
        Tile::Lava
    } else if cell < 0.8 {
        Tile::Ground
    } else {
        Tile::Water
    }
}

fn to_wall() -> Tile {
    let mut rng = rand::rng();
    let percentage = rng.random_range(1..100);
    if percentage < 99 {
        Tile::Wall
    } else {
        let percentage = rng.random_range(1..100);
        if percentage < 50 {
            Tile::Bronze
        } else if percentage < 70 {
            Tile::Silver
        } else if percentage < 85 {
            Tile::Gold
        } else if percentage < 95 {
            Tile::Mytril
        } else {
            Tile::Demonite
        }
    }
}
