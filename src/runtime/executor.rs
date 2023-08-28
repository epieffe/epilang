use std::collections::HashMap;

use thiserror::Error;

use crate::intermediate::constant::Type;
use crate::intermediate::exp::{Exp, FunctionExp};
use crate::intermediate::opcode::{BinaryOpcode, UnaryOpcode};
use crate::runtime::operations::OperationError;

use super::module::Module;
use super::value::{V, Value, Function, Class, Object};
use super::pointer::Ptr;

#[derive(Error, Debug)]
pub enum ExpressionError {
    #[error("OperationError: {0}")]
    OperationError(OperationError),
    #[error("RuntimeError: list index out of range")]
    ListIndexOutofRange,
    #[error("TypeError: {0} is not callable")]
    ValueNotCallable(Type),
    #[error("TypeError: function requires {0} positional argument(s) but {1} was given")]
    WrongArgumentsNumber(usize, usize),
    #[error("TypeError: {0} is not subscriptable")]
    NotSubscriptable(Type),
    #[error("TypeError: {0} indices must be integers, not {1}")]
    IndexTypeError(Type, Type),
    #[error("TypeError: no such field or method {0}")]
    NoSuchFieldOrMethod(String),
    #[error("TypeError: no such field {0}")]
    NoSuchField(String),
}

pub fn evaluate(exp: &Exp, module: &mut Module, stack_start: usize) -> Result<V, ExpressionError> {
    match exp {
        Exp::Constant { value } => {
            Ok(V::Val(Value::from(value)))
        },

        Exp::Variable { scope } => {
            Ok(V::Ptr(module.variables[*scope + stack_start]))
        },

        Exp::Class { id } => {
            Ok(V::Val(Value::Class(*module.classes.get(id).expect("Class not found"))))
        }

        Exp::Concatenation { first, second } => {
            evaluate(first, module, stack_start)?;
            evaluate(second, module, stack_start)
        },

        Exp::BinaryOp { op, arg1, arg2 } => {
            let val1 = evaluate(arg1, module, stack_start)?;
            match op {
                BinaryOpcode::Mul => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    let result = val1.as_ref() * val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::Div => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    let result = val1.as_ref() / val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::Add => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    let result = val1.as_ref() + val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::Sub => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    let result = val1.as_ref() - val2.as_ref();
                    let value = result.map_err(|e| ExpressionError::OperationError(e))?;
                    Ok(V::Val(value))
                },
                BinaryOpcode::And => {
                    if val1.as_bool() {
                        Ok(evaluate(arg2, module, stack_start)?)
                    } else {
                        Ok(val1)
                    }
                },
                BinaryOpcode::Or => {
                    if val1.as_bool() {
                        Ok(val1)
                    } else {
                        Ok(evaluate(arg2, module, stack_start)?)
                    }
                },
                BinaryOpcode::Equals => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() == val2.as_ref())))
                },
                BinaryOpcode::NotEquals => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() != val2.as_ref())))
                },
                BinaryOpcode::Greater => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() > val2.as_ref())))
                },
                BinaryOpcode::GreaterEquals => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() >= val2.as_ref())))
                },
                BinaryOpcode::Lower => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() < val2.as_ref())))
                },
                BinaryOpcode::LowerEquals => {
                    let val2 = evaluate(arg2, module, stack_start)?;
                    Ok(V::Val(Value::Bool(val1.as_ref() <= val2.as_ref())))
                },
            }
        },

        Exp::UnaryOp { op, arg } => {
            let val = evaluate(arg, module, stack_start)?;
            match op {
                UnaryOpcode::Not => {
                    Ok(V::Val(!val.as_ref()))
                },
            }
        },

        Exp::Let { scope: _ } => {
            module.variables.push(Ptr::null());
            Ok(V::Val(Value::Unit))
        },

        Exp::Assignment { left, right } => {
            let right_v: V = evaluate(right, module, stack_start)?;
            let ptr = match right_v {
                V::Ptr(p) => p,
                V::Val(value) => Ptr::from(value),
            };
            match left.as_ref() {
                Exp::Variable { scope } => module.variables[scope + stack_start] = ptr,

                Exp::Subscript { element, index } => {
                    let mut e = evaluate(element, module, stack_start)?;
                    let i = evaluate(index, module, stack_start)?;
                    let value_ptr = subscript(e.as_mut_ref(), i.as_ref())?;
                    *value_ptr = ptr;
                },

                Exp::PropertyAccess { exp, property } => {
                    let mut v = evaluate(exp, module, stack_start)?;
                    match v.as_mut_ref() {
                        Value::Object(obj) => {
                            match obj.get_mut_field(property) {
                                Some(field) => *field = ptr,
                                None => return Err(ExpressionError::NoSuchField(property.clone())),
                            }
                        },
                        _ => return Err(ExpressionError::NoSuchField(property.clone())),
                    }
                }
                // Invalid left-expression in assignments are detected at compile time
                _ => unreachable!(),
            }
            Ok(V::Val(Value::Unit))
        },

        Exp::Block { exp } => {
            let scope = module.variables.len();
            let result = evaluate(exp, module, stack_start);
            module.variables.truncate(scope);
            result
        },

        Exp::Condition { exp, then_block, else_block } => {
            let condition = evaluate(exp, module, stack_start)?;
            let scope = module.variables.len();
            let result = if condition.as_bool() {
                evaluate(then_block, module, stack_start)
            } else {
                evaluate(else_block, module, stack_start)
            };
            module.variables.truncate(scope);
            result
        },

        Exp::While { guard, exp } => {
            loop {
                if !evaluate(guard, module, stack_start)?.as_bool() { break }
                evaluate(exp, module, stack_start)?;
            }
            Ok(V::Val(Value::Unit))
        }

        Exp::List { elements } => {
            let mut list = Vec::with_capacity(elements.len());
            for element in elements {
                let value = match evaluate(element, module, stack_start)? {
                    V::Ptr(p) => p,
                    V::Val(v) => Ptr::from(v),
                };
                list.push(value)
            }
            Ok(V::Val(Value::List(list)))
        }

        Exp::Subscript { element, index } => {
            let mut e = evaluate(element, module, stack_start)?;
            let i = evaluate(index, module, stack_start)?;
            let value_ptr = subscript(e.as_mut_ref(), i.as_ref())?;
            Ok(V::Ptr(*value_ptr))
        }

        Exp::Function(function_exp) => {
            let FunctionExp {
                num_args, external_vars, body
            } = function_exp.as_ref();
            let function = Function {
                num_args: *num_args,
                external_values: Vec::new(),
                body: Box::new(body.clone()),
            };
            let function_ptr = Ptr::from(Value::Function(function));
            match function_ptr.clone().as_mut_ref() {
                Value::Function(fun) => {
                    // Push self reference as external value to enable recursion
                    fun.external_values.push(function_ptr);
                    // Push external values to function stack
                    fun.external_values.append(&mut external_vars.iter().map(|var| {module.variables[*var + stack_start]}).collect())
                },
                _ => unreachable!()
            }
            module.variables.push(function_ptr);
            Ok(V::Ptr(function_ptr))
        },

        Exp::Closure(function_exp) => {
            let FunctionExp {
                num_args, external_vars, body
            } = function_exp.as_ref();
            let external_values = external_vars.iter().map(|var| {module.variables[*var + stack_start]}).collect();
            let function = Function {
                num_args: *num_args,
                external_values: external_values,
                body: Box::new(body.clone()),
            };
            Ok(V::Val(Value::Function(function)))
        },

        Exp::FunctionCall { fun, args } => {
            let fun = evaluate(fun, module, stack_start)?;
            match fun.as_ref() {
                // Function call
                Value::Function(fun) => {
                    let mut args_v = Vec::with_capacity(args.len());
                    for arg in args {
                        let v = evaluate(&arg, module, stack_start)?;
                        args_v.push(v.into_ptr())
                    }
                    call_function(fun, args_v, module)
                },
                // Method call
                Value::Method(method) => {
                    call_method(method.function.as_ref(), method.self_value, args, module, stack_start)
                },
                // Class constructor call
                Value::Class(class) => {
                    // Create object
                    let mut fields = HashMap::with_capacity(class.as_ref().fields.len());
                    for field_name in &class.as_ref().fields {
                        fields.insert(field_name.clone(), Ptr::null());
                    }
                    let object = Value::Object(Object { class: *class, fields});
                    // Call constructor
                    call_method(&class.as_ref().constructor, Ptr::from(&object), args, module, stack_start)?;
                    Ok(V::Val(object))
                },
                _ => Err(ExpressionError::ValueNotCallable(fun.as_ref().get_type()))
            }
        },

        Exp::ClassDef(class_exp) => {
            // Create class
            let class = Class {
                name: class_exp.name.clone(),
                fields: class_exp.fields.clone(),
                constructor: Function {
                    num_args: class_exp.constructor.num_args,
                    external_values: Vec::new(),
                    body: Box::new(class_exp.constructor.body.clone())
                },
                methods: (&class_exp.methods).into_iter().map(|(k, v)| {
                    let function = Function {
                        num_args: v.num_args,
                        external_values: Vec::new(),// Class methods never have external values
                        body: Box::new(v.body.clone()),
                    };
                    (k.clone(), Ptr::from(function))
                }).collect(),
            };
            // Load class in module
            module.classes.insert(class_exp.id, Ptr::from(class));
            Ok(V::Val(Value::Unit))
        },

        Exp::PropertyAccess { exp, property } => {
            let v = evaluate(exp, module, stack_start)?;
            match v.as_ref().get_field(property) {
                // Check if a field with property name exists
                Some(ptr) => Ok(V::Ptr(ptr)),
                // Then check if a method with property name exists
                None => match v.as_ref().get_method(property) {
                    Some(m) => Ok(V::Val(Value::Method(m))),
                    None => Err(ExpressionError::NoSuchFieldOrMethod(property.clone()))
                },
            }
        },
    }
}

fn subscript<'a, 'b>(element: &'a mut Value, index: &'b Value) -> Result<&'a mut Ptr<Value>, ExpressionError> {
    match (element, index) {
        (Value::List(values), Value::Int(i)) => {
            values.get_mut(*i as usize).ok_or(ExpressionError::ListIndexOutofRange)
        },
        (v, Value::Int(_)) => Err(ExpressionError::NotSubscriptable(v.get_type())),
        (v, i) => Err(ExpressionError::IndexTypeError(v.get_type(), i.get_type()))
    }
}

fn call_function(fun: &Function, args: Vec<Ptr<Value>>, module: &mut Module) -> Result<V, ExpressionError> {
    if fun.num_args == args.len() {
        let function_stack_start = module.variables.len();
        // Push external values to variable stack
        module.variables.extend_from_slice(&fun.external_values);
        // Push function args to variable stack
        module.variables.extend(args);
        let result = evaluate(fun.body.as_ref(), module, function_stack_start);
        module.variables.truncate(function_stack_start);
        Ok(result?)
    } else {
        Err(ExpressionError::WrongArgumentsNumber(fun.num_args, args.len()))
    }
}

fn call_method(fun: &Function, self_ptr: Ptr<Value>, args: &Vec<Exp>,  module: &mut Module, stack_start: usize) -> Result<V, ExpressionError> {
    let mut args_v = Vec::with_capacity(args.len() + 1);
    // Push self reference as first method argument
    args_v.push(self_ptr);
    for arg in args {
        let v = evaluate(&arg, module, stack_start)?;
        args_v.push(v.into_ptr())
    }
    call_function(fun, args_v, module)
}
