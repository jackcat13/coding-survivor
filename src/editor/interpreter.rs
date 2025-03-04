use super::grammar::{Expression, Operation, Operator, Primary, Unary};

pub enum InterpreterResult{
    InterpreterNum(f64),
    InterpreterStr(String),
    InterpreterBool(bool),
    InterpreterBang(Box<InterpreterResult>),
    InterpreterNil,
}

#[derive(Debug)]
pub enum InterpreterError{
    ExpressionNotHandled,
    EofShouldNotBeInterpreted,
    InvalidOperationValues,
}

pub fn interpret_expression(expression: &Expression) -> Result<InterpreterResult, InterpreterError> {
    match expression {
        Expression::Operation(Operation::Operation(left, operator, right)) => solve_operation(left, operator, right),
        _ => Err(InterpreterError::ExpressionNotHandled),
    }
}

fn solve_operation(left: &Operation, operator: &super::grammar::Operator, right: &Operation) -> Result<InterpreterResult, InterpreterError> {
    let (left, right) = interpret_operation_parts(left, right);
    match left {
        Ok(left) => match right {
            Ok(right) => match operator {
                super::grammar::Operator::Add => solve_add(left, right),
                super::grammar::Operator::Minus => todo!(),
                super::grammar::Operator::Multiply => todo!(),
                super::grammar::Operator::Divide => todo!(),
                super::grammar::Operator::EqualEqual => todo!(),
                super::grammar::Operator::BangEqual => todo!(),
                super::grammar::Operator::Less => todo!(),
                super::grammar::Operator::LessOrEqual => todo!(),
                super::grammar::Operator::Greater => todo!(),
                super::grammar::Operator::GreaterOrEqual => todo!(),
            },
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}

fn solve_add(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    match left {
        InterpreterResult::InterpreterNum(num_left) => match right {
            InterpreterResult::InterpreterNum(num_right) => Ok(InterpreterResult::InterpreterNum(num_left + num_right)),
            InterpreterResult::InterpreterStr(str_right) => Ok(InterpreterResult::InterpreterStr(num_left.to_string() + &str_right)),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        InterpreterResult::InterpreterStr(_) => todo!(),
        InterpreterResult::InterpreterBool(_) => todo!(),
        InterpreterResult::InterpreterBang(_) => todo!(),
        InterpreterResult::InterpreterNil => todo!(),
    }
}

fn interpret_operation_parts(left: &Operation, right: &Operation) -> (Result<InterpreterResult, InterpreterError>, Result<InterpreterResult, InterpreterError>) {
    let left = match left {
        Operation::Operation(left_nested, operator_nested, right_nested) => solve_operation(left_nested, operator_nested, right_nested),
        Operation::Unary(unary) => solve_unary(unary),
    };
    let right = match right {
        Operation::Operation(left_nested, operator_nested, right_nested) => solve_operation(left_nested, operator_nested, right_nested),
        Operation::Unary(unary) => solve_unary(unary),
    };
    (left, right)
}

fn solve_unary(unary: &Unary) -> Result<InterpreterResult, InterpreterError> {
    match unary {
        Unary::Bang(unary_nested) => {
            match solve_unary(unary_nested) {
                Ok(unary_nested_result) => match unary_nested_result {
                    InterpreterResult::InterpreterBang(bang_result) => Ok(*bang_result),
                    InterpreterResult::InterpreterNum(num_result) => Ok(InterpreterResult::InterpreterBang(Box::new(InterpreterResult::InterpreterNum(num_result)))),
                    InterpreterResult::InterpreterStr(str_result) => Ok(InterpreterResult::InterpreterBang(Box::new(InterpreterResult::InterpreterStr(str_result)))),
                    InterpreterResult::InterpreterBool(_) => todo!(),
                    InterpreterResult::InterpreterNil => todo!(),
                },
                Err(error) => Err(error),
            }
        },
        Unary::Minus(unary_nested) => {
            match solve_unary(unary_nested) {
                Ok(unary_nested_result) => match unary_nested_result {
                    InterpreterResult::InterpreterNum(_) => todo!(),
                    InterpreterResult::InterpreterStr(_) => todo!(),
                    InterpreterResult::InterpreterBang(_) => todo!(),
                    InterpreterResult::InterpreterBool(_) => todo!(),
                    InterpreterResult::InterpreterNil => todo!(),
                },
                Err(error) => Err(error),
            }
        }
        Unary::Primary(primary) => solve_primary(primary),
    }
}

fn solve_primary(primary: &Primary) -> Result<InterpreterResult, InterpreterError> {
    match primary{
        Primary::Number(number) => Ok(InterpreterResult::InterpreterNum(*number)),
        Primary::Str(str) => Ok(InterpreterResult::InterpreterStr(str.to_string())),
        Primary::True => Ok(InterpreterResult::InterpreterBool(true)),
        Primary::False => Ok(InterpreterResult::InterpreterBool(false)),
        Primary::Nil => Ok(InterpreterResult::InterpreterNil),
        Primary::Eof => Err(InterpreterError::EofShouldNotBeInterpreted),
    }
}
