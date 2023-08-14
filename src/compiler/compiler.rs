use std::collections::HashMap;
use thiserror::Error;

use crate::intermediate::exp::Exp;

use super::ast::AST;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Variable {0} is not defined")]
    UndefinedVariable(String),
    #[error("Invalid left expression: {0}")]
    InvalidLeftSideAssignment(String),
}

pub struct Context {
    pub variable_scope: Vec<String>,
    pub variable_map: HashMap<String, usize>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variable_scope: Vec::new(),
            variable_map: HashMap::new(),
        }
    }
}

pub fn compile(ast: &AST) -> Result<Exp, CompilerError> {
    compile_with_context(ast, &mut Context::new())
}

pub fn compile_with_context(ast: &AST, context: &mut Context) -> Result<Exp, CompilerError> {

    match ast {
        AST::Constant(value) => {
            Ok(Exp::Constant { value: value.clone() })
        },

        AST::Identifier(name) => {
            let scope = context.variable_map.get(name).ok_or(CompilerError::UndefinedVariable(name.clone()))?;
            Ok(Exp::Variable { scope: *scope })
        },

        AST::Concatenation { left, right } => {
            let exp1 = compile_with_context(left, context)?;
            let exp2 = compile_with_context(right, context)?;
            Ok(Exp::Concatenation { first: Box::new(exp1), second: Box::new(exp2) })
        },

        AST::BinaryOp(arg1, op, arg2) => {
            let exp1 = compile_with_context(arg1, context)?;
            let exp2 = compile_with_context(arg2, context)?;
            Ok(Exp::BinaryOp { op: *op, arg1: Box::new(exp1), arg2: Box::new(exp2) })
        },

        AST::UnaryOp(op, arg) => {
            let exp = compile_with_context(arg, context)?;
            Ok(Exp::UnaryOp { op: *op, arg: Box::new(exp) })
        },

        AST::Definition(name) => {
            let scope = context.variable_scope.len();
            context.variable_scope.push(name.clone());
            context.variable_map.insert(name.clone(), scope);
            Ok(Exp::Let { scope })
        },

        AST::Assignment(left, right) => {
            let left_exp = compile_with_context(left, context)?;
            let right_exp = compile_with_context(right, context)?;
            match left_exp {
                Exp::Let { scope } => {
                    Ok(Exp::Concatenation {
                        first: Box::new(left_exp),
                        second: Box::new(Exp::Assignment {
                            left: Box::new(Exp::Variable { scope }),
                            right: Box::new(right_exp)
                        })
                    })
                },
                Exp::Variable { scope: _ } => {
                    Ok(Exp::Assignment {
                        left: Box::new(left_exp),
                        right: Box::new(right_exp)
                    })
                },
                _ => Err(CompilerError::InvalidLeftSideAssignment(left.to_string()))
            }
        },

        AST::Block(exp) => {
            let scope = context.variable_scope.len();
            let exp = compile_with_context(exp, context)?;
            while context.variable_scope.len() > scope {
                let var = context.variable_scope.pop().unwrap();
                context.variable_map.remove(&var);
            }
            Ok(exp)
        },

        AST::Condition { exp, then_block, else_block } => {
            let exp = compile_with_context(exp, context)?;
            let scope = context.variable_scope.len();
            // then block
            let then_block = compile_with_context(then_block, context)?;
            while context.variable_scope.len() > scope {
                let var = context.variable_scope.pop().unwrap();
                context.variable_map.remove(&var);
            }
            // else block
            let else_block = compile_with_context(else_block, context)?;
            while context.variable_scope.len() > scope {
                let var = context.variable_scope.pop().unwrap();
                context.variable_map.remove(&var);
            }
            Ok(Exp::Condition {
                exp: Box::new(exp),
                then_block: Box::new(then_block),
                else_block: Box::new(else_block)
            })
        },
    }
}
