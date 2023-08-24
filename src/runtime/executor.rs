use thiserror::Error;

use crate::intermediate::constant::Type;
use crate::intermediate::exp::Exp;
use crate::intermediate::opcode::{BinaryOpcode, UnaryOpcode};
use crate::runtime::operations::OperationError;

use super::value::{V, Value, Pointer, Function};

#[derive(Error, Debug)]
pub enum ExpressionError {
    #[error("OperationError: {0}")]
    OperationError(OperationError),
    #[error("RuntimeError: list index out of range")]
    ListIndexOutofRange(),
    #[error("TypeError: {0} is not callable")]
    ValueNotCallable(Type),
    #[error("TypeError: function requires {0} positional argument(s) but {1} was given")]
    WrongArgumentsNumber(usize, usize),
    #[error("TypeError: {0} is not subscriptable")]
    NotSubscriptable(Type),
    #[error("TypeError: {0} indices must be integers, not {1}")]
    IndexTypeError(Type, Type)
}

pub fn evaluate(exp: &Exp, stack: &mut Vec<Pointer>, stack_start: usize) -> Result<V, ExpressionError> {
    match exp {
        Exp::Constant { value } => {
            Ok(V::Val(Value::from(value)))
        },

        Exp::Variable { scope } => {
            Ok(V::Ptr(stack[*scope + stack_start]))
        },

        Exp::Concatenation { first, second } => {
            evaluate(first, stack, stack_start)?;
            evaluate(second, stack, stack_start)
        },

        Exp::BinaryOp { op, arg1, arg2 } => {
            let val1 = evaluate(arg1, stack, stack_start)?;
            match op {
                BinaryOpcode::Mul => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    let result = val1.as_ref() * val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::Div => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    let result = val1.as_ref() / val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::Add => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    let result = val1.as_ref() + val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::Sub => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    let result = val1.as_ref() - val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::And => {
                    if val1.as_bool() {
                        Ok(evaluate(arg2, stack, stack_start)?)
                    } else {
                        Ok(val1)
                    }
                },
                BinaryOpcode::Or => {
                    if val1.as_bool() {
                        Ok(val1)
                    } else {
                        Ok(evaluate(arg2, stack, stack_start)?)
                    }
                },
                BinaryOpcode::Equals => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() == val2.as_ref())))
                },
                BinaryOpcode::NotEquals => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() != val2.as_ref())))
                },
                BinaryOpcode::Greater => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() > val2.as_ref())))
                },
                BinaryOpcode::GreaterEquals => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() >= val2.as_ref())))
                },
                BinaryOpcode::Lower => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() < val2.as_ref())))
                },
                BinaryOpcode::LowerEquals => {
                    let val2 = evaluate(arg2, stack, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() <= val2.as_ref())))
                },
            }
        },

        Exp::UnaryOp { op, arg } => {
            let val = evaluate(arg, stack, stack_start)?;
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
            let right_v: V = evaluate(right, stack, stack_start)?;
            let ptr = match right_v {
                V::Ptr(p) => p,
                V::Val(value) => Pointer::from(Box::new(value)),
            };
            match left.as_ref() {
                Exp::Variable { scope } => stack[scope + stack_start] = ptr,

                Exp::Subscript { element, index } => {
                    let mut e = evaluate(element, stack, stack_start)?;
                    let i = evaluate(index, stack, stack_start)?;
                    let value_ptr = subscript(e.as_mut_ref(), i.as_ref())?;
                    *value_ptr = ptr;

                }

                _ => panic!("Invalid left-expression in assignments are detected at compile time"),
            }
            Ok(V::Val(Value::Unit))
        },

        Exp::Block { exp } => {
            let scope = stack.len();
            let result = evaluate(exp, stack, stack_start);
            stack.truncate(scope);
            result
        },

        Exp::Condition { exp, then_block, else_block } => {
            let condition = evaluate(exp, stack, stack_start)?;
            let scope = stack.len();
            let result = if condition.as_bool() {
                evaluate(then_block, stack, stack_start)
            } else {
                evaluate(else_block, stack, stack_start)
            };
            stack.truncate(scope);
            result
        },

        Exp::While { guard, exp } => {
            loop {
                if !evaluate(guard, stack, stack_start)?.as_bool() { break }
                evaluate(exp, stack, stack_start)?;
            }
            Ok(V::Val(Value::Unit))
        }

        Exp::List { elements } => {
            let mut list = Vec::with_capacity(elements.len());
            for element in elements {
                let value = match evaluate(element, stack, stack_start)? {
                    V::Ptr(p) => p,
                    V::Val(v) => Pointer::from(Box::new(v)),
                };
                list.push(value)
            }
            Ok(V::Val(Value::List(list)))
        }

        Exp::Subscript { element, index } => {
            let mut e = evaluate(element, stack, stack_start)?;
            let i = evaluate(index, stack, stack_start)?;
            let value_ptr = subscript(e.as_mut_ref(), i.as_ref())?;
            Ok(V::Ptr(*value_ptr))
        }

        Exp::Closure { num_args, exp } => {
            let function = Function {
                num_args: *num_args,
                external_values: Vec::new(),
                body: exp.clone(),
            };
            Ok(V::Val(Value::Function(function)))
        },

        Exp::FunctionCall { fun, args } => {
            let fun = evaluate(fun, stack, stack_start)?;
            match fun.as_ref() {
                Value::Function(f) => {
                    if args.len() == f.num_args {
                        let function_stack_start = stack.len();
                        for arg in args {
                            match evaluate(arg, stack, stack_start)? {
                                V::Ptr(ptr) => stack.push(ptr),
                                V::Val(value) => stack.push(Pointer::from(Box::new(value)))
                            };
                        };
                        let result = evaluate(f.body.as_ref(), stack, function_stack_start)?;
                        stack.truncate(function_stack_start);
                        Ok(result)
                    } else {
                        Err(ExpressionError::WrongArgumentsNumber(f.num_args, args.len()))
                    }
                },
                _ => Err(ExpressionError::ValueNotCallable(fun.as_ref().get_type()))
            }
        },
    }
}

fn subscript<'a, 'b>(element: &'a mut Value, index: &'b Value) -> Result<&'a mut Pointer, ExpressionError> {
    match (element, index) {
        (Value::List(values), Value::Int(i)) => {
            values.get_mut(*i as usize).ok_or(ExpressionError::ListIndexOutofRange())
        },
        (v, Value::Int(_)) => Err(ExpressionError::NotSubscriptable(v.get_type())),
        (v, i) => Err(ExpressionError::IndexTypeError(v.get_type(), i.get_type()))
    }
}
