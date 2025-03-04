use raylib::{
    color::Color,
    ffi::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};

use crate::{
    editor::{grammar::{resolve_ast, AstParseError}, interpreter::{interpret_expression, InterpreterResult}, keyboard::{BACKSPACE, CARRIAGE_RETURN, KEYS_PRESSED}, tokenizer::{get_prompt_tokens, TokenizerError}}, game_state::{EDITOR_STATE, MAP_STATE}, GET_EDITOR_STATE_ERROR, TILE_SIZE
};

pub fn main_scene(rl: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32) {
    let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(thread);
    let x_game_anchor: i32 = width / 3;

    d.clear_background(Color::GRAY);

    editor_processing();
    editor_rendering(&mut d, x_game_anchor, height, x_game_anchor);
    map_rendering(&mut d, x_game_anchor);
}

#[allow(static_mut_refs)]
fn editor_processing() {
    let mut editor_state = EDITOR_STATE.lock().expect(GET_EDITOR_STATE_ERROR);

    if let Some(key) = unsafe { KEYS_PRESSED.pop_back() } {
        if key == BACKSPACE {
            editor_state.buffer.pop();
        } else if key == CARRIAGE_RETURN {
            let prompt: String = editor_state.buffer.iter().collect();
            editor_state.buffer = vec![];
            let tokens = get_prompt_tokens(prompt.clone());
            println!("Tokens for the command :");
            tokens.iter().for_each(|token| {
                println!("{:?}", token);
            });
            editor_state.commands.push(prompt.clone());
            match tokens {
                Ok(tokens) => {
                    println!("AST Expressions for the command :");
                    match resolve_ast(tokens) {
                        Ok(ast) => ast.tree.iter().for_each(|expression| {
                            println!("{:?}", expression);
                            match interpret_expression(expression) {
                                Ok(result) => match result {
                                    InterpreterResult::InterpreterNum(num_result) => editor_state.commands.push(format!("Result : {}", num_result)),
                                    InterpreterResult::InterpreterStr(str_result) => editor_state.commands.push(format!("Result : {}", str_result)),
                                    InterpreterResult::InterpreterBool(_) => todo!(),
                                    InterpreterResult::InterpreterBang(_) => todo!(),
                                    InterpreterResult::InterpreterNil => todo!(),
                                },
                                Err(error) => println!("{:?}", error),
                            }
                        }),
                        Err(error) => match error {
                            AstParseError::TokenInvalidGrammar => editor_state.commands.push("ERR-Invalid grammar for provided command".to_string()),
                            AstParseError::MissingLiteralForNumber => editor_state.commands.push("ERR-Missing value for parsed number".to_string()),
                            AstParseError::MissingLiteralForString => editor_state.commands.push("ERR-Missing value for parsed String".to_string()),
                            AstParseError::MissingLiteralForIdentifier => editor_state.commands.push("ERR-Missing value for parsed Identifier".to_string()),
                            AstParseError::UnaryWithNoValidNextToken => editor_state.commands.push("ERR-Invalid value passed after ! or -".to_string()),
                            AstParseError::InvalidFactorExpressions => editor_state.commands.push("ERR-Invalid values passed to operation".to_string()),
                        },
                    }
                }
                Err(error) => match error {
                    TokenizerError::TokenScanError => editor_state.commands.push("ERR-Some unexpected character used while processing input".to_string()),
                    TokenizerError::StringTokenScanError => editor_state.commands.push("ERR-Invalid String definition while processing input. Any \" must match another \" character".to_string()),
                    TokenizerError::IdentifierMissmatch => editor_state.commands.push("ERR-Invalid identifier, use a valid keyword instead".to_string()), 
                },
            }
        } else {
            editor_state.buffer.push(key);
        }
    }
}

const EDITOR_PROMPT_Y: i32 = 10;
const EDITOR_HISTORY_Y: i32 = 40;
const EDITOR_HISTORY_LINE_HEIGHT: i32 = 30;
const EDITOR_TEXT_X: i32 = 20;
const EDITOR_FONT_SIZE: i32 = 20;

#[allow(static_mut_refs)]
fn editor_rendering(d: &mut RaylibDrawHandle<'_>, x_game_anchor: i32, height: i32, width: i32) {
    let editor_state = EDITOR_STATE.lock().expect(GET_EDITOR_STATE_ERROR);
    d.draw_rectangle(0, 0, x_game_anchor, height, Color::BLACK);
    let input_line: String = editor_state.buffer.iter().collect();
    let input_line = "> ".to_owned() + &input_line;
    d.draw_text(&input_line, EDITOR_TEXT_X, EDITOR_PROMPT_Y, EDITOR_FONT_SIZE, Color::WHITE);
    let mut y_history_position = EDITOR_HISTORY_Y;
    for history_text in editor_state.commands.iter().rev() {
        if y_history_position > height {
            break;
        }
        let (text, color) = resolve_history_text_format(history_text.to_string());
        let character_width = EDITOR_FONT_SIZE as f32 / 1.5;
        let lines = ((text.len() as i32 * character_width as i32) / width) + 1;
        let mut x_index = 0;
        let line_max_width = width / character_width as i32;
        for _ in 0..lines {
            let mut x_index_end = x_index + line_max_width;
            if x_index_end > text.len() as i32 { x_index_end = text.len() as i32 }
            let text_slice = &text[x_index as usize..x_index_end as usize];
            d.draw_text(text_slice, EDITOR_TEXT_X, y_history_position, EDITOR_FONT_SIZE, color);
            x_index += line_max_width;
            y_history_position += EDITOR_HISTORY_LINE_HEIGHT;
        }
    }
}

fn map_rendering(d: &mut RaylibDrawHandle, x_game_anchor: i32) {
    let map = MAP_STATE.lock().expect("Failed to get map state");
    let (mut x, mut y) = (x_game_anchor, 0);
    for line in map.tiles.iter() {
        for tile in line.iter() {
            let color = match tile {
                crate::game_state::Tile::Player => Color::GREEN,
                crate::game_state::Tile::Ground => Color::LIGHTGRAY,
                crate::game_state::Tile::Wall => Color::GRAY,
            };
            d.draw_rectangle_v(
                Vector2 {
                    x: x as f32,
                    y: y as f32,
                },
                Vector2 {
                    x: TILE_SIZE as f32,
                    y: TILE_SIZE as f32,
                },
                color,
            );
            x += 32;
        }
        x = x_game_anchor;
        y += 32;
    }
}

fn resolve_history_text_format(history_text: String) -> (String, Color) {
    if history_text.starts_with("ERR-") {
        let text = history_text.replace("ERR-", "");
        return (text, Color::RED);
    }
    (history_text, Color::WHITE)
}


