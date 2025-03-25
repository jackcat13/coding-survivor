use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::{editor::grammar::{Function, Operation, Primary, Unary}, game_state::{Direction, MoveError, MAP_STATE}};

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
    ]);
}

pub struct FunctionDef {
    pub name: String,
    pub arguments: Vec<String>,
    pub instructions: InstructionsDef,
}

pub enum InstructionsDef {
    Expressions(Vec<Expression>),
    NativeFunction(fn(&Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError>),
}

#[derive(Debug)]
pub enum FunctionError {
    ExpectedArgumentsCount(usize),
    PlayerMoveError(MoveError),
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

fn move_player(
    arguments: &Vec<InterpreterResult>,
    direction: Direction,
) -> Result<InterpreterResult, FunctionError> {
    if !arguments.is_empty() {
        return Err(FunctionError::ExpectedArgumentsCount(0));
    }
    let mut map_state = MAP_STATE.lock().expect("Failed to get map state");
    match map_state.may_move_player(direction) {
        Ok(_) => Ok(InterpreterResult::Nil),
        Err(error) => Err(FunctionError::PlayerMoveError(error)),
    }
}
