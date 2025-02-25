use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: u32,
}

#[derive(Debug)]
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
}

pub enum ParserError{
    TokenScanError, StringTokenScanError,
}

pub fn get_prompt_tokens(prompt: String) -> Result<Vec<Token>, ParserError> {
    let mut tokens = vec![];

    let mut characters = prompt.chars();
    while let Some(character) = characters.next() {
        match character {
            '(' => add_token(TokenType::LEFT_PAREN, character, 0, &mut tokens),
            ')' => add_token(TokenType::RIGHT_PAREN, character, 0, &mut tokens),
            '{' => add_token(TokenType::LEFT_BRACE, character, 0, &mut tokens),
            '}' => add_token(TokenType::RIGHT_BRACE, character, 0, &mut tokens),
            ',' => add_token(TokenType::COMMA, character, 0, &mut tokens),
            '.' => add_token(TokenType::DOT, character, 0, &mut tokens),
            '-' => add_token(TokenType::MINUS, character, 0, &mut tokens),
            '+' => add_token(TokenType::PLUS, character, 0, &mut tokens),
            ';' => add_token(TokenType::SEMICOLON, character, 0, &mut tokens),
            '*' => add_token(TokenType::STAR, character, 0, &mut tokens),
            '/' => add_token(TokenType::SLASH, character, 0, &mut tokens),
            '!' => add_token(resolve_two_chars_type(TokenType::BANG, characters.next()), character, 0, &mut tokens),
            '=' => add_token(resolve_two_chars_type(TokenType::EQUAL, characters.next()), character, 0, &mut tokens),
            '<' => add_token(resolve_two_chars_type(TokenType::LESS, characters.next()), character, 0, &mut tokens),
            '>' => add_token(resolve_two_chars_type(TokenType::GREATER, characters.next()), character, 0, &mut tokens),
            '"' => {
                match resolve_string(&mut characters) {
                    Ok(value) => add_token_with_literal(TokenType::STRING, character, 0, value, &mut tokens),
                    Err(err) => return Err(err),
                }
            },
            ' ' => continue,
            _ => return Err(ParserError::TokenScanError),
        }
    }

    tokens.push(Token{ token_type: TokenType::EOF, lexeme: "".to_string(), literal: None, line: 0 });

    Ok(tokens)
}

fn resolve_string(characters: &mut Chars) -> Result<String, ParserError> {
    let mut result = String::new();
    for character in characters.by_ref() {
        if character != '"' {
            result.push(character);
        } else {
            return Ok(result);
        }
    }
    Err(ParserError::StringTokenScanError)
}

fn resolve_two_chars_type(token_type: TokenType, next_character: Option<char>) -> TokenType {
    if let Some(character) = next_character {
        if character == '=' {
            match token_type {
                TokenType::BANG => TokenType::BANG_EQUAL,
                TokenType::EQUAL => TokenType::EQUAL_EQUAL,
                TokenType::LESS => TokenType::LESS_EQUAL,
                TokenType::GREATER => TokenType::GREATER_EQUAL,
                _ => panic!("Token scanning for 2 characters identifiers can't be called on {:?} token type", token_type),
            }
        } else {
            token_type
        }
    } else {
        token_type
    }
}

fn add_token(token_type: TokenType, character: char, line: u32, tokens: &mut Vec<Token>) {
    tokens.push(Token { token_type, lexeme: character.to_string(), literal: None, line });
}

fn add_token_with_literal(token_type: TokenType, character: char, line: u32, literal: String, tokens: &mut Vec<Token>) {
    tokens.push(Token { token_type, lexeme: character.to_string(), literal: Some(literal), line });
}

