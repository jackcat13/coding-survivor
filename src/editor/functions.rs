use std::{fmt::Arguments, sync::Mutex};

use lazy_static::lazy_static;

use crate::{editor::grammar::{Function, Operation, Primary, Unary}, game_state::{BreakError, Direction, MoveError, MAP_STATE}};

use super::{grammar::Expression, interpreter::InterpreterResult};

lazy_static! {
    pub static ref FUNCTIONS: Mutex<Vec<FunctionDef>> = Mutex::new(vec![
        FunctionDef {
            name: "test".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::Expressions(vec![Expression::Function(Function::Operation(Operation::Unary(Unary::Primary(Primary::Str("Working".to_string())))))])
        },
        FunctionDef {
            name: "moveDown".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(move_down)
        },
        FunctionDef {
            name: "moveUp".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(move_up)
        },
        FunctionDef {
            name: "moveLeft".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(move_left)
        },
        FunctionDef {
            name: "moveRight".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(move_right)
        },
        FunctionDef {
            name: "zoomOut".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(zoom_out)
        },
        FunctionDef {
            name: "zoomIn".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(zoom_in)
        },
        FunctionDef {
            name: "loot".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(loot)
        },
        FunctionDef {
            name: "breakDown".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(break_down)
        },
        FunctionDef {
            name: "breakUp".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(break_up)
        },
        FunctionDef {
            name: "breakLeft".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(break_left)
        },
        FunctionDef {
            name: "breakRight".to_string(),
            arguments: vec![],
            instructions: InstructionsDef::NativeFunction(break_right)
        },
    ]);
}

pub struct FunctionDef {
    pub name: String,
    pub arguments: Vec<String>,
    pub instructions: InstructionsDef,
}

impl FunctionDef {
    pub fn is_matching(&self, input_string: String) -> bool {
        if self.name.starts_with(&input_string) || input_string.starts_with(&self.name) {
            return true;
        }
        false
    }

    pub fn to_complete_string(&self) -> String {
        if self.arguments.is_empty() {
            self.name.clone() + "()"
        } else {
            self.name.clone() + "("
        }
    }
}

pub enum InstructionsDef {
    Expressions(Vec<Expression>),
    NativeFunction(fn(&Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError>),
}

#[derive(Debug)]
pub enum FunctionError {
    ExpectedArgumentsCount(usize),
    PlayerMoveError(MoveError),
    BreakSomethingError(BreakError),
}

fn move_down(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    move_player(arguments, Direction::Down)
}

fn move_up(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    move_player(arguments, Direction::Up)
}

fn move_left(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    move_player(arguments, Direction::Left)
}

fn move_right(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    move_player(arguments, Direction::Right)
}

fn expected_empty_arguments(arguments: &Vec<InterpreterResult>) -> Result<(), FunctionError> {
    if !arguments.is_empty() {
        return Err(FunctionError::ExpectedArgumentsCount(0));
    }
    Ok(())
}

fn move_player(
    arguments: &Vec<InterpreterResult>,
    direction: Direction,
) -> Result<InterpreterResult, FunctionError> {
    expected_empty_arguments(arguments)?;
    let mut map_state = MAP_STATE.lock().expect("Failed to get map state");
    match map_state.may_move_player(direction) {
        Ok(_) => Ok(InterpreterResult::Nil),
        Err(error) => Err(FunctionError::PlayerMoveError(error)),
    }
}

fn break_down(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    break_something(arguments, Direction::Down)
}

fn break_up(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    break_something(arguments, Direction::Up)
}

fn break_left(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    break_something(arguments, Direction::Left)
}

fn break_right(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    break_something(arguments, Direction::Right)
}

fn break_something(
    arguments: &Vec<InterpreterResult>,
    direction: Direction,
) -> Result<InterpreterResult, FunctionError> {
    expected_empty_arguments(arguments)?;
    let mut map_state = MAP_STATE.lock().expect("Failed to get map state");
    match map_state.may_break_something(direction) {
        Ok(_) => Ok(InterpreterResult::Nil),
        Err(error) => Err(FunctionError::BreakSomethingError(error)),
    }
}

fn zoom_out(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    let mut map_state = MAP_STATE.lock().expect("Failed to load map state");
    if map_state.zoom > 0.1 {
        map_state.zoom -= 0.1;
    }
    Ok(InterpreterResult::Nil)
}

fn zoom_in(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    let mut map_state = MAP_STATE.lock().expect("Failed to load map state");
    if map_state.zoom < 2.0 {
        map_state.zoom += 0.1;
    }
    Ok(InterpreterResult::Nil)
}

fn loot(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    let mut map_state = MAP_STATE.lock().expect("Failed to load map state");
    for index in 0..map_state.items.len() {
        if map_state.items[index].position.x == map_state.player.position.x && map_state.items[index].position.y == map_state.player.position.y {
            let inventory_item = map_state.items[index].item.to_inventory_item();
            if map_state.player.inventory.contains(&inventory_item) {
                map_state.player.inventory.iter_mut().filter(|item|{
                    **item == inventory_item
                }).for_each(|item|{
                    item.number += 1;
                });
            } else {
                map_state.player.inventory.push(inventory_item);
            }
            map_state.items.remove(index);
        }
    }
    Ok(InterpreterResult::Nil)
}

