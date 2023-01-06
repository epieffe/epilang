use std::fmt;

use crate::expression::Exp;
use crate::parser::SyntaxError;
use crate::token::Operator;
use crate::value::{Value, StackValue, Function, V};

use Value::Int;
use Value::Bool;

pub struct Error {
    pub msg: String
}

pub fn eval(exp: &Exp) -> Result<V, Error> {
    let mut stack: Vec<StackValue> = Vec::new();
    eval_expression(exp, &mut stack, 0)
}

pub fn eval_expression(exp: &Exp, stack: &mut Vec<StackValue>, stack_start: usize) -> Result<V, Error> {
    match exp {
        Exp::Const(c) => Result::Ok(V::Val(Value::from_const(&c))),

        Exp::Var(x) => {
            Result::Ok(V::Ptr(stack[x.scope + stack_start]))
        },

        Exp::Decl(_, val_exp, exp2) => {
            match eval_expression(val_exp, stack, stack_start)? {
                V::Ptr(ptr) => stack.push(ptr),
                V::Val(value) => stack.push(StackValue::from_box(Box::new(value)))
            };
            let result = eval_expression(exp2, stack, stack_start);
            stack.pop();
            result
        },

        Exp::Assign(var, exp) => {
            match eval_expression(exp, stack, stack_start)? {
                V::Ptr(ptr) => stack[var.scope + stack_start] = ptr,
                V::Val(value) => stack[var.scope + stack_start] = StackValue::from_box(Box::new(value))
            };
            Result::Ok(V::Ptr(StackValue::unit()))
        }

        Exp::IfThenElse(condition, exp1, exp2) => {
            let is_true: bool = eval_expression(condition, stack, stack_start)?.as_bool();
            eval_expression(if is_true {exp1} else {exp2}, stack, stack_start)
        }

        Exp::Function(args, body) => {
            Result::Ok(V::Val(Value::Fn(Function { num_args: args.len(), external_values: Vec::new(), body: body.clone() })))
        },
        Exp::FunctionCall(callable, args) => {
            match eval_expression(callable, stack, stack_start)?.as_ref() {
                Value::Fn(function) => {
                    if args.len() != function.num_args {
                        return Result::Err(Error { msg: format!("Wrong number of arguments. Expected {}, found {}", function.num_args, args.len()) })
                    }
                    let function_stack_start: usize = stack.len();
                    for arg in args {
                        match eval_expression(arg, stack, stack_start)? {
                            V::Ptr(ptr) => stack.push(ptr),
                            V::Val(value) => stack.push(StackValue::from_box(Box::new(value)))
                        };
                    };
                    let result: V = eval_expression(function.body.as_ref(), stack, function_stack_start)?;
                    for _ in 0..function.num_args {
                        stack.pop();
                    }
                    Result::Ok(result)
                },
                _ => return Result::Err(Error{msg: String::from("Expression is not callable")})
            }
        },

        Exp::Seq(exp1, exp2) => {
            eval_expression(exp1, stack, stack_start)?;
            eval_expression(exp2, stack, stack_start)
        },

        Exp::Sum(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(sum(val1.as_ref(), val2.as_ref())?))
        },

        Exp::Sub(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(sub(val1.as_ref(), val2.as_ref())?))
        },

        Exp::Mul(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(mul(val1.as_ref(), val2.as_ref())?))
        },

        Exp::Div(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(div(val1.as_ref(), val2.as_ref())?))
        },

        Exp::Lt(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(lt(val1.as_ref(), val2.as_ref())?))
        },

        Exp::Gt(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(gt(val1.as_ref(), val2.as_ref())?))
        },

        Exp::Eq(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(eq(val1.as_ref(), val2.as_ref())?))
        },

        Exp::Neq(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(neq(val1.as_ref(), val2.as_ref())?))
        },

        Exp::And(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(and(val1.as_ref(), val2.as_ref())?))
        },

        Exp::Or(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack, stack_start)?;
            Result::Ok(V::Val(or(val1.as_ref(), val2.as_ref())?))
        },

        Exp::Not(exp1) => {
            let v = eval_expression(exp1, stack, stack_start)?;
            Result::Ok(V::Val(Value::Bool(!v.as_bool())))
        }
    }
}

fn double_eval(exp1: &Exp, exp2: &Exp, stack: &mut Vec<StackValue>, stack_start: usize) -> Result<(V, V), Error> {
    let v1 = eval_expression(exp1, stack, stack_start)?;
    let v2 = eval_expression(exp2, stack, stack_start)?;
    Result::Ok((v1, v2))
}

fn sum(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Int(i1), Value::Int(i2)) => Result::Ok(Value::Int(i1 + i2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}

fn sub(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Int(i1), Value::Int(i2)) => Result::Ok(Value::Int(i1 - i2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}

fn mul(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Int(i1), Value::Int(i2)) => Result::Ok(Value::Int(i1 * i2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}

fn div(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Int(i1), Value::Int(i2)) => Result::Ok(Value::Int(i1 / i2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}

fn lt(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Int(i1), Value::Int(i2)) => Result::Ok(Value::Bool(i1 < i2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}

fn gt(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Int(i1), Value::Int(i2)) => Result::Ok(Value::Bool(i1 > i2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}

fn eq(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Int(i1), Value::Int(i2)) => Result::Ok(Value::Bool(i1 == i2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}

fn neq(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Int(i1), Value::Int(i2)) => Result::Ok(Value::Bool(i1 != i2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}

fn and(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Bool(b1), Value::Bool(b2)) => Result::Ok(Value::Bool(*b1 && *b2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}

fn or(val1: &Value, val2: &Value) -> Result<Value, Error> {
    match (val1, val2) {
        (Value::Bool(b1), Value::Bool(b2)) => Result::Ok(Value::Bool(*b1 || *b2)),
        _ => Result::Err(Error{msg: format!("Unsupported + operator for values {}, {}",val1, val2)})
    }
}
