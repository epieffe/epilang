use std::collections::HashMap;

use crate::intermediate::exp::{Exp, FunctionExp, ClassExp};

use super::ast::{AST, ClassAST};
use super::error::CompilerError;
use super::frame::{GlobalContext, Frame};

pub fn compile(ast: &AST, frame: &mut Frame, ctx: &mut GlobalContext) -> Result<Exp, CompilerError> {

    match ast {
        AST::Constant(value) => {
            Ok(Exp::Constant { value: value.clone() })
        },

        AST::Identifier(name) => {
            let scope = frame.variable_scope(name)?;
            Ok(Exp::Variable { scope })
        },

        AST::Concatenation { left, right } => {
            let exp1 = compile(left, frame, ctx)?;
            let exp2 = compile(right, frame, ctx)?;
            Ok(Exp::Concatenation { first: Box::new(exp1), second: Box::new(exp2) })
        },

        AST::BinaryOp(arg1, op, arg2) => {
            let exp1 = compile(arg1, frame, ctx)?;
            let exp2 = compile(arg2, frame, ctx)?;
            Ok(Exp::BinaryOp { op: *op, arg1: Box::new(exp1), arg2: Box::new(exp2) })
        },

        AST::UnaryOp(op, arg) => {
            let exp = compile(arg, frame, ctx)?;
            Ok(Exp::UnaryOp { op: *op, arg: Box::new(exp) })
        },

        AST::Definition(name) => {
            let scope = frame.define_variable(name.clone());
            Ok(Exp::Let { scope })
        },

        AST::Assignment(left, right) => {
            let left_exp = compile(left, frame, ctx)?;
            let right_exp = compile(right, frame, ctx)?;
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
                Exp::Variable { scope: _ } | Exp::Subscript { element: _, index: _ } => {
                    Ok(Exp::Assignment {
                        left: Box::new(left_exp),
                        right: Box::new(right_exp)
                    })
                },
                _ => Err(CompilerError::InvalidLeftSideAssignment)
            }
        },

        AST::Block(exp) => {
            let mut sub_frame = Frame::new(frame);
            let exp = compile(exp, &mut sub_frame, ctx)?;
            Ok(Exp::Block { exp: Box::new(exp) })
        },

        AST::Condition { exp, then_block, else_block } => {
            let exp = compile(exp, frame, ctx)?;
            // then block
            let mut then_frame = Frame::new(frame);
            let then_block = compile(then_block, &mut then_frame, ctx)?;
            // else block
            let mut else_frame = Frame::new(frame);
            let else_block = compile(else_block, &mut else_frame, ctx)?;
            Ok(Exp::Condition {
                exp: Box::new(exp),
                then_block: Box::new(then_block),
                else_block: Box::new(else_block)
            })
        },

        AST::While {guard, exp } => {
            let guard = compile(guard, frame, ctx)?;
            let exp = compile(exp, frame, ctx)?;
            Ok(Exp::While { guard: Box::new(guard), exp: Box::new(exp) })
        },

        AST::List { elements } => {
            let mut list = Vec::with_capacity(elements.len());
            for element in elements {
                list.push(compile(element, frame, ctx)?)
            }
            Ok(Exp::List { elements: list })
        },

        AST::Subscript { element, index } => {
            let e = compile(element, frame, ctx)?;
            let i = compile(index, frame, ctx)?;
            Ok(Exp::Subscript { element: Box::new(e), index: Box::new(i) })
        }

        AST::Function(f) => {
            let fn_exp = compile_function(Some(&f.name), &f.args, &f.body, frame, ctx)?;
            // Function is assigned to a new variable in current scope
            frame.define_variable(f.name.clone());
            Ok(Exp::Function(Box::new(fn_exp)))
        },

        AST::Closure { args, exp } => {
            let fn_exp = compile_function(None, args, exp, frame, ctx)?;
            Ok(Exp::Closure(Box::new(fn_exp)))
        },

        AST::FunctionCall { fun, args } => {
            let fun_exp = compile(fun, frame, ctx)?;
            let mut args_exps = Vec::new();
            for arg in args {
                let arg_exp = compile(arg, frame, ctx)?;
                args_exps.push(arg_exp);
            };
            Ok(Exp::FunctionCall { fun: Box::new(fun_exp), args: args_exps })
        },

        AST::Class(class_ast) => {
            let ClassAST {
                name, fields, methods
            } = class_ast.as_ref();
            let mut methods_map = HashMap::with_capacity(methods.len());
            for m in methods {
                let mut args = Vec::with_capacity(m.args.len());
                args.push("self".to_owned());
                m.args.clone_into(&mut args);
                let function_exp = compile_function(None, &args, &m.body, frame, ctx)?;
                methods_map.insert(m.name.clone(), function_exp);
            }
            let id = ctx.define_class(name.clone())?;
            let class_exp = ClassExp {
                id,
                name: name.clone(),
                fields: fields.clone(),
                methods: methods_map,
            };
            Ok(Exp::Class(Box::new(class_exp)))
        }
    }
}

fn compile_function(name: Option<&str>, args: &Vec<String>, body: &AST, _frame: &mut Frame, ctx: &mut GlobalContext) -> Result<FunctionExp, CompilerError> {
    let mut function_frame: Frame = Default::default();
    if name.is_some() {
        // Function is assigned to a variable in its own scope to enable recursion
        function_frame.define_variable(name.unwrap().to_owned());
    }
    for arg in args {
        function_frame.define_variable(arg.clone());
    }
    Ok(FunctionExp {
        num_args: args.len(),
        // TODO: extract external variables from frame
        external_vars: Vec::new(),
        body: compile(body, &mut function_frame, ctx)?
    })
}
