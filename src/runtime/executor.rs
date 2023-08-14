use crate::intermediate::exp::Exp;
use crate::intermediate::opcode::{BinaryOpcode, UnaryOpcode};
use crate::runtime::operations::OperationError;
use thiserror::Error;

use super::value::{V, Value, Pointer};

#[derive(Error, Debug)]
pub enum ExpressionError {
    #[error("OperationError: {0}")]
    OperationError(OperationError),
    #[error("Variable {0} is not defined")]
    UndefinedVariable(String),
    #[error("Invalid left expression: {0}")]
    InvalidLeftSideAssignment(String),
}

pub fn evaluate(exp: &Exp) -> Result<V, ExpressionError> {
    evaluate_with_stack(exp, &mut Vec::new(), 0)
}

pub fn evaluate_with_stack(exp: &Exp, stack: &mut Vec<Pointer>, stack_start: usize) -> Result<V, ExpressionError> {
    match exp {
        Exp::Constant { value } => {
            Ok(V::Val(Value::from(value)))
        },

        Exp::Variable { scope } => {
            Ok(V::Ptr(stack[*scope + stack_start]))
        },

        Exp::Concatenation { first, second } => {
            evaluate_with_stack(first, stack, stack_start)?;
            evaluate_with_stack(second, stack, stack_start)
        },

        Exp::BinaryOp { op, arg1, arg2 } => {
            let val1 = evaluate_with_stack(arg1, stack, stack_start)?;
            match op {
                BinaryOpcode::Mul => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    let result = val1.as_ref() * val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::Div => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    let result = val1.as_ref() / val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::Add => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    let result = val1.as_ref() + val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::Sub => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    let result = val1.as_ref() - val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::And => {
                    if val1.as_bool() {
                        Ok(evaluate_with_stack(arg2, stack, stack_start)?)
                    } else {
                        Ok(val1)
                    }
                },
                BinaryOpcode::Or => {
                    if val1.as_bool() {
                        Ok(val1)
                    } else {
                        Ok(evaluate_with_stack(arg2, stack, stack_start)?)
                    }
                },
                BinaryOpcode::Equals => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() == val2.as_ref())))
                },
                BinaryOpcode::NotEquals => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() != val2.as_ref())))
                },
                BinaryOpcode::Greater => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() > val2.as_ref())))
                },
                BinaryOpcode::GreaterEquals => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() >= val2.as_ref())))
                },
                BinaryOpcode::Lower => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() < val2.as_ref())))
                },
                BinaryOpcode::LowerEquals => {
                    let val2 = evaluate_with_stack(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() <= val2.as_ref())))
                },
            }
        },

        Exp::UnaryOp { op, arg } => {
            let val = evaluate_with_stack(arg, stack, stack_start)?;
            match op {
                UnaryOpcode::Not => {
                    Ok(V::Val(!val.as_ref()))
                },
            }
        },

        Exp::Let { scope: _ } => {
            stack.push(Pointer::unit());
            Ok(V::Val(Value::Unit))
        },

        Exp::Assignment { left, right } => {
            let right_v: V = evaluate_with_stack(right, stack, stack_start)?;
            match left.as_ref() {
                Exp::Variable { scope } => match right_v {
                    V::Ptr(ptr) => stack[scope + stack_start] = ptr,
                    V::Val(value) => stack[scope + stack_start] = Pointer::from(Box::new(value)),
                },

                _ => panic!("Invalid left-expression in assignment"),
            }
            Ok(V::Val(Value::Unit))
        },

        Exp::Block { exp } => {
            let scope = stack.len();
            let result = evaluate_with_stack(exp, stack, stack_start);
            stack.truncate(scope);
            result
        },

        Exp::Condition { exp, then_block, else_block } => {
            let condition = evaluate_with_stack(exp, stack, stack_start)?;
            let scope = stack.len();
            let result = if condition.as_bool() {
                evaluate_with_stack(then_block, stack, stack_start)
            } else {
                evaluate_with_stack(else_block, stack, stack_start)
            };
            stack.truncate(scope);
            result
        },
    }
}
