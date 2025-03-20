use std::{collections::VecDeque, fmt::Display, thread};

use raylib::ffi::{self, KeyboardKey};

use crate::{
    editor::{
        grammar::{resolve_ast, AstParseError},
        interpreter::{interpret_expression, InterpreterResult},
        tokenizer::{get_prompt_tokens, TokenizerError},
    },
    game_state::EDITOR_STATE,
    GET_EDITOR_STATE_ERROR,
};

pub const BACKSPACE: char = '\x08';
pub const CARRIAGE_RETURN: char = '\x13';
pub const ARROW_UP: char = '↑';
pub static mut KEYS_PRESSED: VecDeque<char> = VecDeque::new();

#[allow(static_mut_refs)]
pub fn start_keyboard_thread() {
    thread::spawn(move || loop {
        process_key_pressed();
    });
}

// Accept mutable static references here because concurrent access to push first
// and pop last is expected
#[allow(static_mut_refs)]
#[inline]
pub fn process_key_pressed() {
    let key = unsafe { ffi::GetKeyPressed() };
    if key == KeyboardKey::KEY_BACKSPACE as i32 {
        unsafe { KEYS_PRESSED.push_front(BACKSPACE) };
    } else if key == KeyboardKey::KEY_ENTER as i32 {
        unsafe { KEYS_PRESSED.push_front(CARRIAGE_RETURN) };
    } else if key == KeyboardKey::KEY_UP as i32 {
        unsafe { KEYS_PRESSED.push_front(ARROW_UP) };
    } else {
        //Process actual character in another thread to avoid performance loss
        thread::spawn(move || {
            let key = unsafe { ffi::GetCharPressed() };
            if key > 0 {
                if let Some(character) = char::from_u32(key as u32) {
                    unsafe { KEYS_PRESSED.push_front(character) };
                }
            }
        });
    }
}

#[allow(static_mut_refs)]
pub fn editor_processing() {
    thread::spawn(|| loop {
        let mut editor_state = EDITOR_STATE.lock().expect(GET_EDITOR_STATE_ERROR);

        if let Some(key) = unsafe { KEYS_PRESSED.pop_back() } {
            match key {
                BACKSPACE => {
                    editor_state.buffer.pop();
                }
                ARROW_UP => {
                    editor_state.buffer = vec![];
                    if let Some(history) = editor_state.input_history.clone().last() {
                        for character in history.chars() {
                            editor_state.buffer.push(character);
                        }
                    };
                }
                CARRIAGE_RETURN => process_prompt(&mut editor_state),
                _ => editor_state.buffer.push(key),
            };
        }
    });
}

fn process_prompt(editor_state: &mut std::sync::MutexGuard<'_, crate::game_state::EditorState>) {
    let prompt: String = editor_state.buffer.iter().collect();
    editor_state.buffer = vec![];
    editor_state.commands.push(prompt.clone());
    editor_state.input_history.push(prompt.clone());
    let tokens = get_prompt_tokens(prompt.clone());
    println!("Tokens for the command :");
    tokens.iter().for_each(|token| {
        println!("{:?}", token);
    });
    match tokens {
        Ok(tokens) => {
            println!("AST Expressions for the command :");
            match resolve_ast(tokens) {
                Ok(ast) => ast.tree.iter().for_each(|expression| {
                    println!("{:?}", expression);
                    match interpret_expression(expression) {
                        Ok(result) => match result {
                            InterpreterResult::Num(num_result) => editor_result_message(editor_state, &num_result),
                            InterpreterResult::Str(str_result) => editor_result_message(editor_state, &str_result),
                            InterpreterResult::Bool(bool_result) => editor_result_message(editor_state, &bool_result),
                            InterpreterResult::Nil => (),
                            _ => println!("Unexpected expression result"),
                        },
                        Err(error) => println!("{:?}", error),
                    }
                }),
                Err(error) => match error {
                    AstParseError::TokenInvalidGrammar => editor_error_message(editor_state, &"Invalid grammar for provided command"),
                    AstParseError::MissingLiteralForNumber => editor_error_message(editor_state, &"Missing value for parsed number"),
                    AstParseError::MissingLiteralForString => editor_error_message(editor_state, &"Missing value for parsed String"),
                    AstParseError::MissingLiteralForIdentifier => editor_error_message(editor_state, &"Missing value for parsed Identifier"),
                    AstParseError::UnaryWithNoValidNextToken => editor_error_message(editor_state, &"Invalid value passed after ! or -"),
                    AstParseError::InvalidFactorExpressions => editor_error_message(editor_state, &"Invalid values passed to operation"),
                    AstParseError::LabelWithNoValidNextToken => editor_error_message(editor_state, &"Invalid values passed after label"),
                    AstParseError::InvalidTokensInGroup => editor_error_message(editor_state, &"Invalid values passed to () group"),
                },
            }
        }
        Err(error) => match error {
            TokenizerError::TokenScanError => editor_error_message(editor_state, &"Some unexpected character used while processing input"),
            TokenizerError::StringTokenScanError => editor_error_message(editor_state, &"Invalid String definition while processing input. Any \" must match another \" character"),
            TokenizerError::IdentifierMissmatch => editor_error_message(editor_state, &"Invalid identifier, use a valid keyword instead"),
            TokenizerError::InvalidFunctionSyntax => editor_error_message(editor_state, &"Invalid function syntax"),
            TokenizerError::NoIdentifierNorFunctionError => editor_error_message(editor_state, &"No matching keyword nor function"),
        },
    };
}

fn editor_result_message(
    editor_state: &mut std::sync::MutexGuard<'_, crate::game_state::EditorState>,
    message: &dyn Display,
) {
    editor_state.commands.push(format!("RES-Result : {}", message));
}

fn editor_error_message(
    editor_state: &mut std::sync::MutexGuard<'_, crate::game_state::EditorState>,
    message: &dyn Display,
) {
    editor_state.commands.push(format!("ERR-{}", message));
}
