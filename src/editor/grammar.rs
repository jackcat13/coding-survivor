use super::tokenizer::{Literal, Token, TokenType};

#[derive(Debug)]
pub enum AstParseError {
    TokenInvalidGrammar, MissingLiteralForNumber,
    MissingLiteralForString,
    MissingLiteralForIdentifier,
}

#[derive(Debug)]
pub struct Ast {
    pub tree: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Primary(Primary),
}

#[derive(Clone, Debug)]
pub enum Primary {
    Number(f64),
    Str(String),
    True,
    False,
    Nil,
    Child(Box<Expression>),
    Eof,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Add, Minus, Multiply, Divide, EqualEqual, BangEqual, Less, LessOrEqual, Greater, GreaterOrEqual
}

pub fn resolve_ast(tokens: Vec<Token>) -> Result<Ast, AstParseError> {
    let mut ast = Ast { tree: vec![] };

    let tokens_iter = tokens.iter();
    for token in tokens_iter {
        let expression = token_to_expression(token);
        match expression {
            Ok(expression) => ast.tree.push(expression),
            Err(error) => return Err(error),
        }
    }

    Ok(ast)
}

fn token_to_expression(token: &Token) -> Result<Expression, AstParseError> {
    match token.token_type {
        // TODO other grammar rules

        // LITERALS
        TokenType::NUMBER => match token.literal.clone() {
            Some(Literal::Num(literal)) => Ok(Expression::Primary(Primary::Number(literal))),
            None => Err(AstParseError::MissingLiteralForNumber),
            _ => Err(AstParseError::MissingLiteralForNumber),
        },
        TokenType::STRING => match token.literal.clone() {
            Some(Literal::Str(literal)) => Ok(Expression::Primary(Primary::Str(literal))),
            None => Err(AstParseError::MissingLiteralForString),
            _ => Err(AstParseError::MissingLiteralForString),
        },
        TokenType::IDENTIFIER => match token.literal.clone() {
            Some(Literal::Identifier(TokenType::TRUE)) => Ok(Expression::Primary(Primary::True)),
            Some(Literal::Identifier(TokenType::FALSE)) => Ok(Expression::Primary(Primary::False)),
            Some(Literal::Identifier(TokenType::NIL)) => Ok(Expression::Primary(Primary::Nil)),
            None => Err(AstParseError::MissingLiteralForIdentifier),
            _ => Err(AstParseError::MissingLiteralForIdentifier),
        },

        // EOF
        TokenType::EOF => Ok(Expression::Primary(Primary::Eof)),

        _ => Err(AstParseError::TokenInvalidGrammar),
    }
}

fn to_operator(token2: &Token) -> Operator {
    match token2.token_type {
        TokenType::PLUS => Operator::Add,
        TokenType::MINUS => Operator::Minus,
        TokenType::STAR => Operator::Multiply,
        TokenType::SLASH => Operator::Divide,
        TokenType::EQUAL_EQUAL => Operator::EqualEqual,
        TokenType::BANG_EQUAL => Operator::BangEqual,
        TokenType::LESS => Operator::Less,
        TokenType::LESS_EQUAL => Operator::LessOrEqual,
        TokenType::GREATER => Operator::Greater,
        TokenType::GREATER_EQUAL => Operator::GreaterOrEqual,
        _ => panic!("Other operator types should have not been parsed"),
    }
}

fn is_token_operator(token: &Token) -> bool {
    token.token_type == TokenType::PLUS ||
    token.token_type == TokenType::MINUS ||
    token.token_type == TokenType::STAR ||
    token.token_type == TokenType::SLASH ||
    token.token_type == TokenType::EQUAL_EQUAL ||
    token.token_type == TokenType::BANG_EQUAL ||
    token.token_type == TokenType::LESS ||
    token.token_type == TokenType::LESS_EQUAL ||
    token.token_type == TokenType::GREATER ||
    token.token_type == TokenType::GREATER_EQUAL
}

