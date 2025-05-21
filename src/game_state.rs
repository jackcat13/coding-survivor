use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable, utils::*};

use std::{
    ops::{Deref, DerefMut},
    sync::Mutex,
    time::{Duration, SystemTime},
};

use rand::prelude::Rng;
use raylib::{collision, ffi::Vector2};

use crate::{
    animation::Animation,
    item::{InventoryItem, Item, MapItem, Pickaxe},
    GAME_HEIGHT, GAME_WIDTH,
};

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
        animation_state: DEFAULT_ANIMATION,
        light_vision: 7.0,
        inventory: vec![],
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
    pub inventory: Vec<InventoryItem>,
}

impl Player {
    pub fn is_ready(&self) -> bool {
        match self.animation_state.cooldown {
            Some(cooldown) => {
                if cooldown > 0.0 {
                    return false;
                }
                return true;
            }
            None => true,
        }
    }
}

pub struct AnimationState {
    pub current_frame: u32,
    pub status: Status,
    pub cooldown: Option<f32>,
    pub target: Option<Vector2>,
}

pub const DEFAULT_ANIMATION: AnimationState = AnimationState {
    current_frame: 0,
    status: Status::Idle,
    cooldown: None,
    target: None,
};

impl AnimationState {
    pub(crate) fn next_frame(&mut self, frame_number: u32) {
        if self.current_frame < frame_number {
            self.current_frame += 1;
        } else {
            self.current_frame = 0;
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum Status {
    Idle,
    Breaking,
}

#[derive(Debug)]
pub enum MoveError {
    HitWall,
    NoTiles,
    PlayerBusy,
}

#[derive(Debug)]
pub enum BreakError {
    Nothing,
    Unbreakable,
    PlayerBusy,
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
        if !self.player.is_ready() {
            return Err(MoveError::PlayerBusy);
        }
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

    pub fn may_break_something(&mut self, direction: Direction) -> Result<(), BreakError> {
        let player_position = self.player.position;
        let break_target = match direction {
            Direction::Up => Vector2 {
                x: player_position.x,
                y: player_position.y - 1.0,
            },
            Direction::Down => Vector2 {
                x: player_position.x,
                y: player_position.y + 1.0,
            },
            Direction::Left => Vector2 {
                x: player_position.x - 1.0,
                y: player_position.y,
            },
            Direction::Right => Vector2 {
                x: player_position.x + 1.0,
                y: player_position.y,
            },
        };
        match self.tiles.get_mut(break_target.y as usize) {
            Some(line) => match line.get_mut(break_target.x as usize) {
                Some(tile) => match tile {
                    Tile::Tree => {
                        if !self.player.is_ready() {
                            return Err(BreakError::PlayerBusy);
                        }
                        self.player.animation_state = AnimationState {
                            current_frame: 0,
                            status: Status::Breaking,
                            cooldown: Some(3.0),
                            target: Some(break_target),
                        };
                        return Ok(());
                    }
                    _ => Err(BreakError::Unbreakable),
                },
                None => Err(BreakError::Nothing),
            },
            None => Err(BreakError::Nothing),
        }
    }

    pub fn spawn_item(&mut self, position: &Vector2, item: Box<dyn Item>) {
        self.items.push(MapItem {
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
    Tree = 10,
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
        Tile::Tree => "Tree".to_string(),
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
    let seed_map = rng.random_range(0..u32::MAX);
    let hasher_map = PermutationTable::new(seed_map);
    let map: NoiseMap = PlaneMapBuilder::new_fn(|point| perlin_2d(point.into(), &hasher_map))
        .set_size(width, height)
        .set_x_bounds(-50.0, 50.0)
        .set_y_bounds(-50.0, 50.0)
        .build();
    let hasher_trees = PermutationTable::new(seed_map);
    let trees: NoiseMap = PlaneMapBuilder::new_fn(|point| perlin_2d(point.into(), &hasher_trees))
        .set_size(width, height)
        .set_x_bounds(-50.0, 50.0)
        .set_y_bounds(-50.0, 50.0)
        .build();
    let map: Vec<f64> = map.iter().copied().collect();
    let trees: Vec<f64> = trees.iter().copied().collect();
    let mut trees_iter = trees.iter();
    let rows: Vec<Vec<Tile>> = (0..height as i32)
        .map(|line| {
            let index = line as usize * width;
            let range = index..index + width - 1;
            let row: Vec<Tile> = map[range]
                .iter()
                .map(|noise_value| to_tile(*noise_value))
                .map(|tile| {
                    may_be_tree(
                        tile,
                        trees_iter
                            .next()
                            .expect("Failed to get noise from trees map"),
                    )
                })
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

fn may_be_tree(tile: Tile, noise_value: &f64) -> Tile {
    if noise_value < &-0.5 && tile == Tile::Ground {
        return Tile::Tree;
    }
    tile
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
