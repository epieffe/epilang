use crate::compiler::ast::{AST};
use crate::intermediate::constant::Constant;
use crate::intermediate::opcode::BinaryOpcode;
use crate::intermediate::opcode::UnaryOpcode;
use crate::runtime::frame::{Frame, VariableError};
use crate::operations::{
    conjunction, disjunction, equals, greater, greater_equals, lower, lower_equals, not_equals,
    OperationError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExpressionError {
    #[error("Unable to evalutate expression {0}: {1}")]
    VariableError(String, VariableError),
    #[error("Unable to evalutate expression {0}: {1}")]
    OperationError(String, OperationError),
    #[error("Variable {0} is not defined")]
    UndefinedVariable(String),
    #[error("Invalid left expression: {0}")]
    InvalidLeftSideAssignment(String),
}

pub fn evalutate_expression(mut frame: Frame, expr: &AST) -> Result<(Constant, Frame), ExpressionError> {
    match expr {
        AST::Constant(n) => Ok((n.clone(), frame)),

        AST::Identifier(variable) => {
            let value = frame
                .variable_value(variable)
                .map_err(|e| ExpressionError::VariableError(expr.to_string(), e))?;
            Ok((value, frame))
        },

        AST::Concatenation { left, right } => {
            let (_, frame) = evalutate_expression(frame, left)?;
            let (value, frame) = evalutate_expression(frame, right)?;
            Ok((value, frame))
        }

        AST::BinaryOp(exp1, opcode, exp2) => {
            let (value_1, frame) = evalutate_expression(frame, exp1)?;
            let (value_2, frame) = evalutate_expression(frame, exp2)?;
            let result = match opcode {
                BinaryOpcode::Mul => value_1 * value_2,
                BinaryOpcode::Div => value_1 / value_2,
                BinaryOpcode::Add => value_1 + value_2,
                BinaryOpcode::Sub => value_1 - value_2,
                BinaryOpcode::Conj => conjunction(value_1, value_2),
                BinaryOpcode::Disj => disjunction(value_1, value_2),
                BinaryOpcode::Equals => equals(value_1, value_2),
                BinaryOpcode::NotEquals => not_equals(value_1, value_2),
                BinaryOpcode::Greater => greater(value_1, value_2),
                BinaryOpcode::GreaterEquals => greater_equals(value_1, value_2),
                BinaryOpcode::Lower => lower(value_1, value_2),
                BinaryOpcode::LowerEquals => lower_equals(value_1, value_2),
            };
            let value = result.map_err(|e| ExpressionError::OperationError(expr.to_string(), e))?;
            Ok((value, frame))
        }

        AST::UnaryOp(op, exp) => {
            let (value, frame) = evalutate_expression(frame, exp)?;
            let value = match op {
                UnaryOpcode::Not => !value,
            };
            Ok((value, frame))
        }

        AST::Definition(identifier) => {
            frame.define_variable(identifier.clone(), Constant::Int(0));
            Ok((Constant::Int(0), frame))
        }

        AST::Assignment(left , right) => {
            match left.as_ref() {
                AST::Identifier(var) => {
                    let (value, mut frame) = evalutate_expression(frame, right)?;
                    frame.assign_value(&var, value)?;
                    Ok((Constant::Int(0), frame))
                }
                AST::Definition(var) => {
                    let (value, mut frame) = evalutate_expression(frame, right)?;
                    frame.define_variable(var.clone(), value);
                    Ok((Constant::Int(0), frame))
                }
                _ => Err(ExpressionError::InvalidLeftSideAssignment(left.to_string()))
            }
        }

        AST::Block(exp) => {
            let frame = Frame::new(Box::new(frame));
            let (value, mut frame) = evalutate_expression(frame, exp)?;
            Ok((value, *frame.take_parent().unwrap()))
        }

        AST::Condition { exp, then_block, else_block } => {
            let (condition, frame) = evalutate_expression(frame, exp)?;
            let frame = Frame::new(Box::new(frame));
            let branch = if condition.as_bool() { then_block } else { else_block };
            let (value, mut frame) = evalutate_expression(frame, branch)?;
            Ok((value, *frame.take_parent().unwrap()))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::lr_lang;
    use crate::intermediate::constant::Constant;
    use crate::runtime::executor::evalutate_expression;
    use crate::Frame;
    use rstest::*;

    #[rstest]
    #[case("1 + 2 * 3 - 4", Constant::Int(3))]
    #[case("!0", Constant::Int(-1))]
    #[case("!-1", Constant::Int(0))]
    #[case("(1 + 2) * (3 - 4)", Constant::Int(-3))]
    #[case("true || false", Constant::Bool(true))]
    #[case("true && false", Constant::Bool(false))]
    #[case("!false", Constant::Bool(true))]
    #[case("!true", Constant::Bool(false))]
    #[case("2 < 3", Constant::Bool(true))]
    #[case("2 <= 3", Constant::Bool(true))]
    #[case("2 <= 3", Constant::Bool(true))]
    #[case("2 >= 2", Constant::Bool(true))]
    #[case("2 >= 1", Constant::Bool(true))]
    #[case("2 == 2", Constant::Bool(true))]
    #[case("2 != 2", Constant::Bool(false))]
    #[case("2 != 3", Constant::Bool(true))]
    #[case("\"abc\" == \"abc\"", Constant::Bool(true))]
    #[case("\"abc\" < \"xyz\"", Constant::Bool(true))]
    #[case("\"abc\" <= \"xyz\"", Constant::Bool(true))]
    #[case("\"abc\" >= \"xyz\"", Constant::Bool(false))]
    #[case("true && false || true || true && false", Constant::Bool(true))]
    #[case("true && (false || true || true) && false", Constant::Bool(false))]
    #[case("\"abc \" + 5.5", Constant::String("abc 5.5".to_owned()))]
    #[case("2 == 2 && 3 == 3", Constant::Bool(true))]
    #[case("100 * 2 == 200 && 120 > 120 - 1", Constant::Bool(true))]
    #[case("100 * 2 < 200 || 120 <= 120 - 1", Constant::Bool(false))]
    #[case("!(100 * 2 < 200) && !(120 <= 120 - 1)", Constant::Bool(true))]
    fn test_evalutate_expression(#[case] expression: &str, #[case] expected: Constant) {
        let parsed = lr_lang::ExprParser::new()
            .parse(expression)
            .expect("Unable to parse expression");
        let mut root_frame = Frame::default();
        let (value, frame) = evalutate_expression(root_frame, parsed.as_ref()).unwrap();
        assert_eq!(expected, value)
    }
}
