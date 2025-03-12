use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::game_state::{MoveError, MAP_STATE};

use super::{grammar::Expression, interpreter::InterpreterResult};

lazy_static! {
    pub static ref FUNCTIONS: Mutex<Vec<FunctionDef>> = Mutex::new(vec![FunctionDef {
        name: "moveDown".to_string(),
        arguments: vec![],
        instructions: InstructionsDef::NativeFunction(move_down)
    }]);
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

fn move_down(arguments: &Vec<InterpreterResult>) -> Result<InterpreterResult, FunctionError> {
    if arguments.len() != 0 {
        return Err(FunctionError::ExpectedArgumentsCount(0));
    }
    let mut map_state = MAP_STATE.lock().expect("Failed to get map state");
    match map_state.may_move_player_down() {
        Ok(_) => Ok(InterpreterResult::Nil),
        Err(error) => Err(FunctionError::PlayerMoveError(error)),
    }
}

#[derive(Debug)]
pub enum FunctionError {
    ExpectedArgumentsCount(usize),
    PlayerMoveError(MoveError),
}
