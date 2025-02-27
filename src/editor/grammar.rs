use super::tokenizer::{Literal, Token, TokenType};

#[derive(Debug)]
pub enum AstParseError {
    InvalidExpressionForBinaryOperation,
}

#[derive(Debug)]
pub struct Ast {
    pub tree: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: Operator,
    },
    Literal {
        value: Literal,
    },
}

#[derive(Clone, Debug)]
pub enum Operator {
    Add, Minus, Multiply, Divide, EqualEqual, BangEqual, Less, LessOrEqual, Greater, GreaterOrEqual
}

pub fn resolve_ast(tokens: Vec<Token>) -> Result<Ast, AstParseError> {
    let mut ast = Ast { tree: vec![] };

    let mut token_index = 0;
    while token_index < tokens.len() {
        let token = tokens.get(token_index).expect("Should be able to resolve first token as < len");
        if let Some(token2) = tokens.get(token_index+1) {
            if let Some(token3) = tokens.get(token_index+2) {
                match is_binary(token, token2, token3) {
                    Ok(result) => {
                        if result {
                            let expression = Expression::Binary{
                                left: to_literal(token),
                                right: to_literal(token3),
                                operator: to_operator(token2),
                            };
                            ast.tree.push(expression);
                            token_index += 3;
                            continue;
                        }
                    },
                    Err(error) => return Err(error),
                }
            }
        }
        token_index += 1;
    }

    Ok(ast)
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

fn to_literal(token: &Token) -> Box<Expression> {
    Box::new(Expression::Literal { value: token.literal.clone().expect("Literal value must be present for Literal type") })
}

fn is_binary(token: &Token, token2: &Token, token3: &Token) -> Result<bool, AstParseError> {
    if is_token_operator(token2) {
        if is_token_operator(token) && is_token_operator(token3) {
            return Err(AstParseError::InvalidExpressionForBinaryOperation);
        }
        return Ok(true);
    }
    Ok(false)
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

