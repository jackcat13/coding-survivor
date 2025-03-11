use std::slice::Iter;

use super::tokenizer::{Literal, Token, TokenType};

#[derive(Debug)]
pub enum AstParseError {
    TokenInvalidGrammar,
    MissingLiteralForNumber,
    MissingLiteralForString,
    MissingLiteralForIdentifier,
    UnaryWithNoValidNextToken,
    InvalidFactorExpressions,
    LabelWithNoValidNextToken,
    InvalidTokensInGroup,
}

#[derive(Debug)]
pub struct Ast {
    pub tree: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Function(Function),
}

#[derive(Clone, Debug)]
pub enum Function {
    Group(Vec<Expression>),
    Operation(Operation),
}

#[derive(Clone, Debug)]
pub enum Operation {
    Operation(Box<Operation>, Operator, Box<Operation>),
    Unary(Unary),
}

#[derive(Clone, Debug)]
pub enum Unary {
    Bang(Box<Unary>),
    Minus(Box<Unary>),
    Primary(Primary),
}

#[derive(Clone, Debug)]
pub enum Primary {
    Number(f64),
    Str(String),
    True,
    False,
    Nil,
    Eof,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Minus,
    Multiply,
    Divide,
    EqualEqual,
    BangEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

pub fn resolve_ast(tokens: Vec<Token>) -> Result<Ast, AstParseError> {
    let mut previous_expression: Option<Expression> = None;
    let mut ast = Ast { tree: vec![] };

    let mut tokens_iter = tokens.iter();
    while let Some(token) = tokens_iter.next() {
        match token_to_expression(token, &mut tokens_iter, &previous_expression) {
            Ok(expression) => {
                if let Expression::Function(Function::Operation(Operation::Operation(_, _, _))) =
                    expression.clone()
                {
                    ast.tree.pop();
                }
                ast.tree.push(expression.clone());
                previous_expression = Some(expression);
            }
            Err(error) => return Err(error),
        }
    }

    Ok(ast)
}

fn token_to_expression(
    token: &Token,
    tokens: &mut Iter<Token>,
    previous_expression: &Option<Expression>,
) -> Result<Expression, AstParseError> {
    // OPERATIONS
    if is_token_operator(token) {
        if let Some(previous_expression) = previous_expression {
            let mut tokens_peek = tokens.clone().peekable();
            if let Some(right) = tokens_peek.peek() {
                tokens.next();
                match token_to_expression(right, tokens, &None) {
                    Ok(right_expression) => {
                        if let Expression::Function(Function::Operation(factor_left)) =
                            previous_expression
                        {
                            if let Expression::Function(Function::Operation(factor_right)) =
                                right_expression
                            {
                                return Ok(Expression::Function(Function::Operation(
                                    Operation::Operation(
                                        Box::new(factor_left.clone()),
                                        to_operator(token),
                                        Box::new(factor_right),
                                    ),
                                )));
                            }
                        }
                        return Err(AstParseError::InvalidFactorExpressions);
                    }
                    Err(error) => return Err(error),
                }
            }
        }
    }

    match token.token_type {
        // LABELS
        TokenType::LABEL => {
            if let Some(next_token) = tokens.next() {
                match token_to_expression(next_token, tokens, &None) {
                    Ok(expression) => match expression {
                        Expression::Function(Function::Group(group)) => {
                            Ok(Expression::Function(Function::Group(group)))
                        }
                        _ => Err(AstParseError::LabelWithNoValidNextToken),
                    },
                    Err(error) => Err(error),
                }
            } else {
                Err(AstParseError::LabelWithNoValidNextToken)
            }
        }

        // GROUP
        TokenType::LEFT_PAREN => {
            let mut expressions: Vec<Expression> = vec![];
            while let Some(next_token) = tokens.next() {
                match token_to_expression(next_token, tokens, &None) {
                    Ok(expression) => {
                        expressions.push(expression);
                        if let Some(next_next_token) = tokens.next() {
                            match next_next_token.token_type {
                                TokenType::COMMA => continue,
                                TokenType::RIGHT_PAREN => return Ok(Expression::Function(Function::Group(expressions))),
                                _ => return Err(AstParseError::InvalidTokensInGroup),
                            }
                        }
                    },
                    Err(error) => return Err(error),
                }
            }
            Err(AstParseError::InvalidTokensInGroup)
        }

        // UNARY
        TokenType::BANG => {
            if let Some(next_token) = tokens.next() {
                match token_to_expression(next_token, tokens, &None) {
                    Ok(expression) => match expression {
                        Expression::Function(Function::Operation(Operation::Unary(unary))) => {
                            Ok(Expression::Function(Function::Operation(Operation::Unary(
                                Unary::Bang(Box::new(unary)),
                            ))))
                        }
                        _ => Err(AstParseError::UnaryWithNoValidNextToken),
                    },
                    Err(error) => Err(error),
                }
            } else {
                Err(AstParseError::UnaryWithNoValidNextToken)
            }
        }
        TokenType::MINUS => {
            if let Some(next_token) = tokens.next() {
                match token_to_expression(next_token, tokens, &None) {
                    Ok(expression) => match expression {
                        Expression::Function(Function::Operation(Operation::Unary(unary))) => {
                            Ok(Expression::Function(Function::Operation(Operation::Unary(
                                Unary::Minus(Box::new(unary)),
                            ))))
                        }
                        _ => Err(AstParseError::UnaryWithNoValidNextToken),
                    },
                    Err(error) => Err(error),
                }
            } else {
                Err(AstParseError::UnaryWithNoValidNextToken)
            }
        }

        // LITERALS
        TokenType::NUMBER => match token.literal.clone() {
            Some(Literal::Num(literal)) => Ok(Expression::Function(Function::Operation(
                Operation::Unary(Unary::Primary(Primary::Number(literal))),
            ))),
            None => Err(AstParseError::MissingLiteralForNumber),
            _ => Err(AstParseError::MissingLiteralForNumber),
        },
        TokenType::STRING => match token.literal.clone() {
            Some(Literal::Str(literal)) => Ok(Expression::Function(Function::Operation(
                Operation::Unary(Unary::Primary(Primary::Str(literal))),
            ))),
            None => Err(AstParseError::MissingLiteralForString),
            _ => Err(AstParseError::MissingLiteralForString),
        },
        TokenType::IDENTIFIER => match token.literal.clone() {
            Some(Literal::Identifier(TokenType::TRUE)) => Ok(Expression::Function(
                Function::Operation(Operation::Unary(Unary::Primary(Primary::True))),
            )),
            Some(Literal::Identifier(TokenType::FALSE)) => Ok(Expression::Function(
                Function::Operation(Operation::Unary(Unary::Primary(Primary::False))),
            )),
            Some(Literal::Identifier(TokenType::NIL)) => Ok(Expression::Function(
                Function::Operation(Operation::Unary(Unary::Primary(Primary::Nil))),
            )),
            None => Err(AstParseError::MissingLiteralForIdentifier),
            _ => Err(AstParseError::MissingLiteralForIdentifier),
        },

        // EOF
        TokenType::EOF => Ok(Expression::Function(Function::Operation(Operation::Unary(
            Unary::Primary(Primary::Eof),
        )))),

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
    token.token_type == TokenType::PLUS
        || token.token_type == TokenType::MINUS
        || token.token_type == TokenType::STAR
        || token.token_type == TokenType::SLASH
        || token.token_type == TokenType::EQUAL_EQUAL
        || token.token_type == TokenType::BANG_EQUAL
        || token.token_type == TokenType::LESS
        || token.token_type == TokenType::LESS_EQUAL
        || token.token_type == TokenType::GREATER
        || token.token_type == TokenType::GREATER_EQUAL
}
