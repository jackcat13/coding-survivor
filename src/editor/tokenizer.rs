use std::{any::Any, char, collections::HashMap, str::Chars};

use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32,
}

#[derive(Clone, Debug)]
pub enum Literal {
    Str(String),
    Num(f64),
    Identifier(TokenType),
    Label(String),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum TokenType{
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE, COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR, PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE, EOF,

    // REFENCES
    LABEL,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = vec![
        ("and", TokenType::AND),
        ("class", TokenType::CLASS),
        ("else", TokenType::ELSE),
        ("false", TokenType::FALSE),
        ("fun", TokenType::FUN),
        ("for", TokenType::FOR),
        ("if", TokenType::IF),
        ("nil", TokenType::NIL),
        ("or", TokenType::OR),
        ("print", TokenType::PRINT),
        ("return", TokenType::RETURN),
        ("super", TokenType::SUPER),
        ("this", TokenType::THIS),
        ("true", TokenType::TRUE),
        ("var", TokenType::VAR),
        ("while", TokenType::WHILE),
    ].into_iter().collect();
}

#[derive(Debug)]
pub enum TokenizerError{
    TokenScanError,
    StringTokenScanError,
    IdentifierMissmatch,
    InvalidFunctionSyntax,
    NoIdentifierNorFunctionError,
}

pub fn get_prompt_tokens(prompt: String) -> Result<Vec<Token>, TokenizerError> {
    let line = 0;
    let mut tokens = vec![];
    let mut characters = prompt.chars();

    while let Some(character) = characters.next() {
        match character {
            '(' => add_token(TokenType::LEFT_PAREN, character, line, &mut tokens),
            ')' => add_token(TokenType::RIGHT_PAREN, character, line, &mut tokens),
            '{' => add_token(TokenType::LEFT_BRACE, character, line, &mut tokens),
            '}' => add_token(TokenType::RIGHT_BRACE, character, line, &mut tokens),
            ',' => add_token(TokenType::COMMA, character, line, &mut tokens),
            '.' => add_token(TokenType::DOT, character, line, &mut tokens),
            '-' => add_token(TokenType::MINUS, character, line, &mut tokens),
            '+' => add_token(TokenType::PLUS, character, line, &mut tokens),
            ';' => add_token(TokenType::SEMICOLON, character, line, &mut tokens),
            '*' => add_token(TokenType::STAR, character, line, &mut tokens),
            '/' => add_token(TokenType::SLASH, character, line, &mut tokens),
            '!' => add_token(resolve_two_chars_type(TokenType::BANG, &mut characters), character, line, &mut tokens),
            '=' => add_token(resolve_two_chars_type(TokenType::EQUAL, &mut characters), character, line, &mut tokens),
            '<' => add_token(resolve_two_chars_type(TokenType::LESS, &mut characters), character, line, &mut tokens),
            '>' => add_token(resolve_two_chars_type(TokenType::GREATER, &mut characters), character, line, &mut tokens),
            '"' => {
                match resolve_string(character, &mut characters) {
                    Ok(value) => add_token_with_literal(TokenType::STRING, character, line, Literal::Str(value), &mut tokens),
                    Err(err) => return Err(err),
                }
            },
            ' ' => continue,
            _ => {
                if character.is_ascii_digit() {
                    match resolve_number(character, &mut characters) {
                        Ok(value) => add_token_with_literal(TokenType::NUMBER, character, line, Literal::Num(value), &mut tokens),
                        Err(err) => return Err(err),
                    }
                    continue;
                } else if character.is_alphanumeric() {
                    match resolve_identifier(character, &mut characters) {
                        Ok(value) => add_token_with_literal(TokenType::IDENTIFIER, character, line, Literal::Identifier(value), &mut tokens),
                        Err(_) => match resolve_label(character, &mut characters) {
                            Ok(label) => tokens.push(Token { token_type: TokenType::LABEL, lexeme: "label".to_string(), literal: Some(label), line }),
                            Err(_) => return Err(TokenizerError::NoIdentifierNorFunctionError),
                        },
                    }
                } else {
                    return Err(TokenizerError::TokenScanError);
                }
            }
        }
    }

    tokens.push(Token{ token_type: TokenType::EOF, lexeme: "".to_string(), literal: None, line });

    Ok(tokens)
}

fn resolve_label(first_character: char, characters: &mut Chars<'_>) -> Result<Literal, TokenizerError> {
    let mut label_name = String::new();
    let mut next_number = 0;
    let mut characters_peek = characters.clone().peekable();
    label_name.push(first_character);
    for character in characters_peek.by_ref() {
        if character.is_alphanumeric() {
            label_name.push(character);
            next_number += 1;
        } else {
            break;
        }
    }
    for _ in 0..next_number {
        characters.next();
    }
    Ok(Literal::Label(label_name))
}

fn resolve_identifier(first_value: char, characters: &mut Chars<'_>) -> Result<TokenType, TokenizerError> {
    let mut identifier = String::new();
    let mut next_number = 0;
    identifier.push(first_value);
    for character in characters.clone().peekable() {
        if character.is_alphanumeric() {
            identifier.push(character);
            next_number += 1;
        } else {
            break;
        }
    }
    match KEYWORDS.get(identifier.as_str()) {
        Some(token) => {
            for _ in 0..next_number {
                characters.next();
            }
            Ok(*token)
        },
        None => Err(TokenizerError::IdentifierMissmatch),
    }
}

fn resolve_number(first_value: char, characters: &mut Chars) -> Result<f64, TokenizerError> {
    let mut result = String::new();
    result.push(first_value);
    for character in characters.clone().peekable() {
        if character.is_ascii_digit() || character == '.' {
            result.push(character);
            characters.next();
        } else {
            break;
        }
    }
    Ok(result.parse::<f64>().expect("Error while parsing token from String to f64"))
}

fn resolve_string(first_value: char, characters: &mut Chars) -> Result<String, TokenizerError> {
    let mut result = String::new();
    result.push(first_value);
    for character in characters.by_ref() {
        if character != '"' {
            result.push(character);
        } else {
            return Ok(result);
        }
    }
    Err(TokenizerError::StringTokenScanError)
}

fn resolve_two_chars_type(token_type: TokenType, characters: &mut Chars) -> TokenType {
    let mut next_character = characters.clone().peekable();
    let result = if let Some(character) = next_character.peek() {
        if *character == '=' {
            match token_type {
                TokenType::BANG => TokenType::BANG_EQUAL,
                TokenType::EQUAL => TokenType::EQUAL_EQUAL,
                TokenType::LESS => TokenType::LESS_EQUAL,
                TokenType::GREATER => TokenType::GREATER_EQUAL,
                _ => token_type
            }
        } else {
            token_type
        }
    } else {
        token_type
    };
    if result != token_type{
        characters.next();
    }
    result
}

fn add_token(token_type: TokenType, character: char, line: u32, tokens: &mut Vec<Token>) {
    tokens.push(Token { token_type, lexeme: character.to_string(), literal: None, line });
}

fn add_token_with_literal(token_type: TokenType, character: char, line: u32, literal: Literal, tokens: &mut Vec<Token>) {
    tokens.push(Token { token_type, lexeme: character.to_string(), literal: Some(literal), line });
}

