use super::grammar::{Expression, Function, Operation, Primary, Unary};

pub enum InterpreterResult{
    Num(f64),
    Str(String),
    Bool(bool),
    Bang(Box<InterpreterResult>),
    Nil,
}

#[derive(Debug)]
pub enum InterpreterError{
    ExpressionNotHandled,
    EofShouldNotBeInterpreted,
    InvalidOperationValues,
    UnexpectedLatelyInterpretedBang,
}

pub fn interpret_expression(expression: &Expression) -> Result<InterpreterResult, InterpreterError> {
    match expression {
        Expression::Function(Function::Operation(Operation::Operation(left, operator, right))) => solve_operation(left, operator, right),
        _ => Err(InterpreterError::ExpressionNotHandled),
    }
}

fn solve_operation(left: &Operation, operator: &super::grammar::Operator, right: &Operation) -> Result<InterpreterResult, InterpreterError> {
    let (left, right) = interpret_operation_parts(left, right);
    match left {
        Ok(left) => match right {
            Ok(right) => match operator {
                super::grammar::Operator::Add => solve_add(left, right),
                super::grammar::Operator::Minus => solve_minus(left, right),
                super::grammar::Operator::Multiply => solve_multiplication(left, right),
                super::grammar::Operator::Divide => solve_division(left, right),
                super::grammar::Operator::EqualEqual => solve_equal_equal(left, right),
                super::grammar::Operator::BangEqual => solve_bang_equal(left, right),
                super::grammar::Operator::Less => solve_less(left, right, false),
                super::grammar::Operator::LessOrEqual => solve_less(left, right, true),
                super::grammar::Operator::Greater => solve_greater(left, right, false),
                super::grammar::Operator::GreaterOrEqual => solve_greater(left, right, true),
            },
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}

fn solve_greater(left: InterpreterResult, right: InterpreterResult, or_equal: bool) -> Result<InterpreterResult, InterpreterError> {
    let false_result = Ok(InterpreterResult::Bool(false));
    match left {
        InterpreterResult::Num(left_num) => match right {
            InterpreterResult::Num(right_num) => 
                if or_equal {
                    Ok(InterpreterResult::Bool(left_num >= right_num))
                } else {
                    Ok(InterpreterResult::Bool(left_num > right_num))
                },
            InterpreterResult::Str(_) => false_result,
            InterpreterResult::Bool(_) => false_result,
            InterpreterResult::Bang(_) => false_result,
            InterpreterResult::Nil => false_result,
        },
        InterpreterResult::Str(left_str) => match right {
            InterpreterResult::Num(_) => false_result,
            InterpreterResult::Str(right_str) => 
                if or_equal {
                    Ok(InterpreterResult::Bool(left_str.len() >= right_str.len()))
                } else {
                    Ok(InterpreterResult::Bool(left_str.len() > right_str.len()))
                },
            InterpreterResult::Bool(_) => false_result,
            InterpreterResult::Bang(_) => false_result,
            InterpreterResult::Nil => false_result,
        },
        _ => Err(InterpreterError::InvalidOperationValues),
    }
}

fn solve_less(left: InterpreterResult, right: InterpreterResult, or_equal: bool) -> Result<InterpreterResult, InterpreterError> {
    let false_result = Ok(InterpreterResult::Bool(false));
    match left {
        InterpreterResult::Num(left_num) => match right {
            InterpreterResult::Num(right_num) => 
                if or_equal {
                    Ok(InterpreterResult::Bool(left_num <= right_num))
                } else {
                    Ok(InterpreterResult::Bool(left_num < right_num))
                },
            InterpreterResult::Str(_) => false_result,
            InterpreterResult::Bool(_) => false_result,
            InterpreterResult::Bang(_) => false_result,
            InterpreterResult::Nil => false_result,
        },
        InterpreterResult::Str(left_str) => match right {
            InterpreterResult::Num(_) => false_result,
            InterpreterResult::Str(right_str) => 
                if or_equal {
                    Ok(InterpreterResult::Bool(left_str.len() <= right_str.len()))
                } else {
                    Ok(InterpreterResult::Bool(left_str.len() < right_str.len()))
                },
            InterpreterResult::Bool(_) => false_result,
            InterpreterResult::Bang(_) => false_result,
            InterpreterResult::Nil => false_result,
        },
        _ => Err(InterpreterError::InvalidOperationValues),
    }
}

fn solve_bang_equal(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    let true_result = Ok(InterpreterResult::Bool(true));
    match left {
        InterpreterResult::Num(left_num) => match right {
            InterpreterResult::Num(right_num) => Ok(InterpreterResult::Bool(left_num != right_num)),
            InterpreterResult::Str(_) => true_result,
            InterpreterResult::Bool(_) => true_result,
            InterpreterResult::Bang(_) => true_result,
            InterpreterResult::Nil => true_result,
        },
        InterpreterResult::Str(left_str) => match right {
            InterpreterResult::Num(_) => true_result,
            InterpreterResult::Str(right_str) => Ok(InterpreterResult::Bool(!left_str.eq(&right_str))),
            InterpreterResult::Bool(_) => true_result,
            InterpreterResult::Bang(_) => true_result,
            InterpreterResult::Nil => true_result,
        },
        InterpreterResult::Bool(left_bool) => match right {
            InterpreterResult::Num(_) => true_result,
            InterpreterResult::Str(_) => true_result,
            InterpreterResult::Bool(right_bool) => Ok(InterpreterResult::Bool(left_bool != right_bool)),
            InterpreterResult::Bang(_) => true_result,
            InterpreterResult::Nil => true_result,
        },
        InterpreterResult::Bang(_) => Err(InterpreterError::UnexpectedLatelyInterpretedBang),
        InterpreterResult::Nil => match right {
            InterpreterResult::Num(_) => true_result,
            InterpreterResult::Str(_) => true_result,
            InterpreterResult::Bool(_) => true_result,
            InterpreterResult::Bang(_) => true_result,
            InterpreterResult::Nil => Ok(InterpreterResult::Bool(false)),
        }
    }
}

fn solve_equal_equal(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    let false_result = Ok(InterpreterResult::Bool(false));
    match left {
        InterpreterResult::Num(left_num) => match right {
            InterpreterResult::Num(right_num) => Ok(InterpreterResult::Bool(left_num == right_num)),
            InterpreterResult::Str(_) => false_result,
            InterpreterResult::Bool(_) => false_result,
            InterpreterResult::Bang(_) => false_result,
            InterpreterResult::Nil => false_result,
        },
        InterpreterResult::Str(left_str) => match right {
            InterpreterResult::Num(_) => false_result,
            InterpreterResult::Str(right_str) => Ok(InterpreterResult::Bool(left_str.eq(&right_str))),
            InterpreterResult::Bool(_) => false_result,
            InterpreterResult::Bang(_) => false_result,
            InterpreterResult::Nil => false_result,
        },
        InterpreterResult::Bool(left_bool) => match right {
            InterpreterResult::Num(_) => false_result,
            InterpreterResult::Str(_) => false_result,
            InterpreterResult::Bool(right_bool) => Ok(InterpreterResult::Bool(left_bool == right_bool)),
            InterpreterResult::Bang(_) => false_result,
            InterpreterResult::Nil => false_result,
        },
        InterpreterResult::Bang(_) => Err(InterpreterError::UnexpectedLatelyInterpretedBang),
        InterpreterResult::Nil => match right {
            InterpreterResult::Num(_) => false_result,
            InterpreterResult::Str(_) => false_result,
            InterpreterResult::Bool(_) => false_result,
            InterpreterResult::Bang(_) => false_result,
            InterpreterResult::Nil => Ok(InterpreterResult::Bool(true)),
        },
    }
}

fn solve_division(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    match left {
        InterpreterResult::Num(num_left) => match right {
            InterpreterResult::Num(num_right) => Ok(InterpreterResult::Num(num_left / num_right)),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        _ => Err(InterpreterError::InvalidOperationValues),
    }
}

fn solve_multiplication(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    match left {
        InterpreterResult::Num(num_left) => match right {
            InterpreterResult::Num(num_right) => Ok(InterpreterResult::Num(num_left * num_right)),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        _ => Err(InterpreterError::InvalidOperationValues),
    }
}

fn solve_minus(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    match left {
        InterpreterResult::Num(num_left) => match right {
            InterpreterResult::Num(num_right) => Ok(InterpreterResult::Num(num_left - num_right)),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        _ => Err(InterpreterError::InvalidOperationValues),
    }
}

fn solve_add(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    match left {
        InterpreterResult::Num(num_left) => match right {
            InterpreterResult::Num(num_right) => Ok(InterpreterResult::Num(num_left + num_right)),
            InterpreterResult::Str(str_right) => Ok(InterpreterResult::Str(num_left.to_string() + &str_right)),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        InterpreterResult::Str(str_left) => match right {
            InterpreterResult::Num(num_right) => Ok(InterpreterResult::Str(str_left + &num_right.to_string())),
            InterpreterResult::Str(str_right) => Ok(InterpreterResult::Str(str_left + &str_right)),
            InterpreterResult::Bool(bool_right) => Ok(InterpreterResult::Str(str_left + &bool_right.to_string())),
            InterpreterResult::Nil => Ok(InterpreterResult::Str(str_left + "nil")),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        _ => Err(InterpreterError::InvalidOperationValues),
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
                    InterpreterResult::Bang(bang_result) => Ok(*bang_result),
                    InterpreterResult::Num(num_result) => Ok(InterpreterResult::Bang(Box::new(InterpreterResult::Num(num_result)))),
                    InterpreterResult::Str(str_result) => Ok(InterpreterResult::Bang(Box::new(InterpreterResult::Str(str_result)))),
                    InterpreterResult::Bool(bool_result) => 
                        if bool_result {
                            Ok(InterpreterResult::Bool(false))
                        } else {
                            Ok(InterpreterResult::Bool(true))
                        },
                    _ => Err(InterpreterError::InvalidOperationValues),
                },
                Err(error) => Err(error),
            }
        },
        Unary::Minus(unary_nested) => {
            match solve_unary(unary_nested) {
                Ok(unary_nested_result) => match unary_nested_result {
                    InterpreterResult::Num(num_result) => Ok(InterpreterResult::Num(-num_result)),
                    _ => Err(InterpreterError::InvalidOperationValues),
                },
                Err(error) => Err(error),
            }
        }
        Unary::Primary(primary) => solve_primary(primary),
    }
}

fn solve_primary(primary: &Primary) -> Result<InterpreterResult, InterpreterError> {
    match primary{
        Primary::Number(number) => Ok(InterpreterResult::Num(*number)),
        Primary::Str(str) => Ok(InterpreterResult::Str(str.to_string())),
        Primary::True => Ok(InterpreterResult::Bool(true)),
        Primary::False => Ok(InterpreterResult::Bool(false)),
        Primary::Nil => Ok(InterpreterResult::Nil),
        Primary::Eof => Err(InterpreterError::EofShouldNotBeInterpreted),
    }
}
