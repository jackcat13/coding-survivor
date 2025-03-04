use super::grammar::{Expression, Operation, Primary, Unary};

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
    UnexpectedLatelyInterpretedBang,
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
                super::grammar::Operator::Minus => solve_minus(left, right),
                super::grammar::Operator::Multiply => solve_multiplication(left, right),
                super::grammar::Operator::Divide => solve_division(left, right),
                super::grammar::Operator::EqualEqual => solve_equal_equal(left, right),
                super::grammar::Operator::BangEqual => solve_bang_equal(left, right),
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

fn solve_bang_equal(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    let true_result = Ok(InterpreterResult::InterpreterBool(true));
    match left {
        InterpreterResult::InterpreterNum(left_num) => match right {
            InterpreterResult::InterpreterNum(right_num) => Ok(InterpreterResult::InterpreterBool(left_num != right_num)),
            InterpreterResult::InterpreterStr(_) => true_result,
            InterpreterResult::InterpreterBool(_) => true_result,
            InterpreterResult::InterpreterBang(_) => true_result,
            InterpreterResult::InterpreterNil => true_result,
        },
        InterpreterResult::InterpreterStr(left_str) => match right {
            InterpreterResult::InterpreterNum(_) => true_result,
            InterpreterResult::InterpreterStr(right_str) => Ok(InterpreterResult::InterpreterBool(!left_str.eq(&right_str))),
            InterpreterResult::InterpreterBool(_) => true_result,
            InterpreterResult::InterpreterBang(_) => true_result,
            InterpreterResult::InterpreterNil => true_result,
        },
        InterpreterResult::InterpreterBool(left_bool) => match right {
            InterpreterResult::InterpreterNum(_) => true_result,
            InterpreterResult::InterpreterStr(_) => true_result,
            InterpreterResult::InterpreterBool(right_bool) => Ok(InterpreterResult::InterpreterBool(left_bool != right_bool)),
            InterpreterResult::InterpreterBang(_) => true_result,
            InterpreterResult::InterpreterNil => true_result,
        },
        InterpreterResult::InterpreterBang(_) => Err(InterpreterError::UnexpectedLatelyInterpretedBang),
        InterpreterResult::InterpreterNil => match right {
            InterpreterResult::InterpreterNum(_) => true_result,
            InterpreterResult::InterpreterStr(_) => true_result,
            InterpreterResult::InterpreterBool(_) => true_result,
            InterpreterResult::InterpreterBang(_) => true_result,
            InterpreterResult::InterpreterNil => Ok(InterpreterResult::InterpreterBool(false)),
        }
    }
}

fn solve_equal_equal(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    let false_result = Ok(InterpreterResult::InterpreterBool(false));
    match left {
        InterpreterResult::InterpreterNum(left_num) => match right {
            InterpreterResult::InterpreterNum(right_num) => Ok(InterpreterResult::InterpreterBool(left_num == right_num)),
            InterpreterResult::InterpreterStr(_) => false_result,
            InterpreterResult::InterpreterBool(_) => false_result,
            InterpreterResult::InterpreterBang(_) => false_result,
            InterpreterResult::InterpreterNil => false_result,
        },
        InterpreterResult::InterpreterStr(left_str) => match right {
            InterpreterResult::InterpreterNum(_) => false_result,
            InterpreterResult::InterpreterStr(right_str) => Ok(InterpreterResult::InterpreterBool(left_str.eq(&right_str))),
            InterpreterResult::InterpreterBool(_) => false_result,
            InterpreterResult::InterpreterBang(_) => false_result,
            InterpreterResult::InterpreterNil => false_result,
        },
        InterpreterResult::InterpreterBool(left_bool) => match right {
            InterpreterResult::InterpreterNum(_) => false_result,
            InterpreterResult::InterpreterStr(_) => false_result,
            InterpreterResult::InterpreterBool(right_bool) => Ok(InterpreterResult::InterpreterBool(left_bool == right_bool)),
            InterpreterResult::InterpreterBang(_) => false_result,
            InterpreterResult::InterpreterNil => false_result,
        },
        InterpreterResult::InterpreterBang(_) => Err(InterpreterError::UnexpectedLatelyInterpretedBang),
        InterpreterResult::InterpreterNil => match right {
            InterpreterResult::InterpreterNum(_) => false_result,
            InterpreterResult::InterpreterStr(_) => false_result,
            InterpreterResult::InterpreterBool(_) => false_result,
            InterpreterResult::InterpreterBang(_) => false_result,
            InterpreterResult::InterpreterNil => Ok(InterpreterResult::InterpreterBool(true)),
        },
    }
}

fn solve_division(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    match left {
        InterpreterResult::InterpreterNum(num_left) => match right {
            InterpreterResult::InterpreterNum(num_right) => Ok(InterpreterResult::InterpreterNum(num_left / num_right)),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        _ => Err(InterpreterError::InvalidOperationValues),
    }
}

fn solve_multiplication(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    match left {
        InterpreterResult::InterpreterNum(num_left) => match right {
            InterpreterResult::InterpreterNum(num_right) => Ok(InterpreterResult::InterpreterNum(num_left * num_right)),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        _ => Err(InterpreterError::InvalidOperationValues),
    }
}

fn solve_minus(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    match left {
        InterpreterResult::InterpreterNum(num_left) => match right {
            InterpreterResult::InterpreterNum(num_right) => Ok(InterpreterResult::InterpreterNum(num_left - num_right)),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        _ => Err(InterpreterError::InvalidOperationValues),
    }
}

fn solve_add(left: InterpreterResult, right: InterpreterResult) -> Result<InterpreterResult, InterpreterError> {
    match left {
        InterpreterResult::InterpreterNum(num_left) => match right {
            InterpreterResult::InterpreterNum(num_right) => Ok(InterpreterResult::InterpreterNum(num_left + num_right)),
            InterpreterResult::InterpreterStr(str_right) => Ok(InterpreterResult::InterpreterStr(num_left.to_string() + &str_right)),
            _ => Err(InterpreterError::InvalidOperationValues),
        },
        InterpreterResult::InterpreterStr(str_left) => match right {
            InterpreterResult::InterpreterNum(num_right) => Ok(InterpreterResult::InterpreterStr(str_left + &num_right.to_string())),
            InterpreterResult::InterpreterStr(str_right) => Ok(InterpreterResult::InterpreterStr(str_left + &str_right)),
            InterpreterResult::InterpreterBool(bool_right) => Ok(InterpreterResult::InterpreterStr(str_left + &bool_right.to_string())),
            InterpreterResult::InterpreterNil => Ok(InterpreterResult::InterpreterStr(str_left + "nil")),
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
                    InterpreterResult::InterpreterBang(bang_result) => Ok(*bang_result),
                    InterpreterResult::InterpreterNum(num_result) => Ok(InterpreterResult::InterpreterBang(Box::new(InterpreterResult::InterpreterNum(num_result)))),
                    InterpreterResult::InterpreterStr(str_result) => Ok(InterpreterResult::InterpreterBang(Box::new(InterpreterResult::InterpreterStr(str_result)))),
                    InterpreterResult::InterpreterBool(bool_result) => 
                        if bool_result {
                            Ok(InterpreterResult::InterpreterBool(false))
                        } else {
                            Ok(InterpreterResult::InterpreterBool(true))
                        },
                    _ => Err(InterpreterError::InvalidOperationValues),
                },
                Err(error) => Err(error),
            }
        },
        Unary::Minus(unary_nested) => {
            match solve_unary(unary_nested) {
                Ok(unary_nested_result) => match unary_nested_result {
                    InterpreterResult::InterpreterNum(num_result) => Ok(InterpreterResult::InterpreterNum(-num_result)),
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
        Primary::Number(number) => Ok(InterpreterResult::InterpreterNum(*number)),
        Primary::Str(str) => Ok(InterpreterResult::InterpreterStr(str.to_string())),
        Primary::True => Ok(InterpreterResult::InterpreterBool(true)),
        Primary::False => Ok(InterpreterResult::InterpreterBool(false)),
        Primary::Nil => Ok(InterpreterResult::InterpreterNil),
        Primary::Eof => Err(InterpreterError::EofShouldNotBeInterpreted),
    }
}
