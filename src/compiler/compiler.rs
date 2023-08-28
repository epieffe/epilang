use std::collections::HashMap;

use crate::intermediate::exp::{Exp, FunctionExp, ClassExp};

use super::ast::AST;
use super::error::CompilerError;
use super::context::CompilerContext;

pub fn compile(ast: &AST, ctx: &mut CompilerContext) -> Result<Exp, CompilerError> {

    match ast {
        AST::Constant(value) => {
            Ok(Exp::Constant { value: value.clone() })
        },

        AST::Identifier(name) => {
            match ctx.variable_scope(name) {
                // If identifier matches a variable name return variable expression
                Some(scope) => Ok(Exp::Variable { scope }),
                // Else if matches a class name return class expression
                None => match ctx.class_id(name) {
                    Some(id) => Ok(Exp::Class { id }),
                    None => Err(CompilerError::UnknownIdentifier(name.clone())),
                },
            }
        },

        AST::Concatenation { left, right } => {
            let exp1 = compile(left, ctx)?;
            let exp2 = compile(right, ctx)?;
            Ok(Exp::Concatenation { first: Box::new(exp1), second: Box::new(exp2) })
        },

        AST::BinaryOp(arg1, op, arg2) => {
            let exp1 = compile(arg1, ctx)?;
            let exp2 = compile(arg2, ctx)?;
            Ok(Exp::BinaryOp { op: *op, arg1: Box::new(exp1), arg2: Box::new(exp2) })
        },

        AST::UnaryOp(op, arg) => {
            let exp = compile(arg, ctx)?;
            Ok(Exp::UnaryOp { op: *op, arg: Box::new(exp) })
        },

        AST::Definition(name) => {
            let scope = ctx.define_variable(name.clone());
            Ok(Exp::Let { scope })
        },

        AST::Assignment(left, right) => {
            let left_exp = compile(left, ctx)?;
            let right_exp = compile(right, ctx)?;
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
                Exp::Variable { scope: _ } |
                Exp::Subscript { element: _, index: _ } |
                Exp::PropertyAccess { exp: _, property: _ } => {
                    Ok(Exp::Assignment {
                        left: Box::new(left_exp),
                        right: Box::new(right_exp)
                    })
                },
                _ => Err(CompilerError::InvalidLeftSideAssignment)
            }
        },

        AST::Block(exp) => {
            let exp = compile_block(exp, ctx, false)?;
            Ok(Exp::Block { exp: Box::new(exp) })
        },

        AST::Condition { exp, then_block, else_block } => {
            let exp = compile(exp, ctx)?;
            // then block
            let then_block = compile_block(then_block, ctx, false)?;
            // else block
            let else_block = compile_block(else_block, ctx, false)?;
            Ok(Exp::Condition {
                exp: Box::new(exp),
                then_block: Box::new(then_block),
                else_block: Box::new(else_block)
            })
        },

        AST::While {guard, exp } => {
            let guard = compile(guard, ctx)?;
            let exp = compile(exp, ctx)?;
            Ok(Exp::While { guard: Box::new(guard), exp: Box::new(exp) })
        },

        AST::List { elements } => {
            let mut list = Vec::with_capacity(elements.len());
            for element in elements {
                list.push(compile(element, ctx)?)
            }
            Ok(Exp::List { elements: list })
        },

        AST::Subscript { element, index } => {
            let e = compile(element, ctx)?;
            let i = compile(index, ctx)?;
            Ok(Exp::Subscript { element: Box::new(e), index: Box::new(i) })
        }

        AST::Function(f) => {
            let fn_exp = compile_function(Some(&f.name), &f.args, &f.body, ctx)?;
            // Function is assigned to a new variable in current scope
            ctx.define_variable(f.name.clone());
            Ok(Exp::Function(Box::new(fn_exp)))
        },

        AST::Closure { args, exp } => {
            let fn_exp = compile_function(None, args, exp, ctx)?;
            Ok(Exp::Closure(Box::new(fn_exp)))
        },

        AST::FunctionCall { fun, args } => {
            let fun_exp = compile(fun, ctx)?;
            let mut args_exps = Vec::new();
            for arg in args {
                let arg_exp = compile(arg, ctx)?;
                args_exps.push(arg_exp);
            };
            Ok(Exp::FunctionCall { fun: Box::new(fun_exp), args: args_exps })
        },

        AST::Class(class_ast) => {
            // Build class constructor (if present)
            let constructor = class_ast.as_ref().constructor.as_ref().map(|fun| {
                let mut args = Vec::with_capacity(fun.args.len());
                args.push("self".to_owned()); // Push self as implicit first argument in constructor
                args.extend_from_slice(&fun.args);
                compile_function(None, &args, &fun.body, ctx)
            }).transpose()?;
            // Build class methods
            let mut methods = HashMap::with_capacity(class_ast.as_ref().methods.len());
            for m in &class_ast.as_ref().methods {
                let mut args = Vec::with_capacity(m.args.len());
                args.push("self".to_owned()); // Push self as implicit first argument in methods
                args.extend_from_slice(&m.args);
                let function_exp = compile_function(None, &args, &m.body, ctx)?;
                methods.insert(m.name.clone(), function_exp);
            }
            // Build class
            let class_exp = ClassExp {
                id: ctx.define_class(class_ast.as_ref().name.clone())?,
                name: class_ast.as_ref().name.clone(),
                fields: class_ast.as_ref().fields.clone(),
                constructor: constructor.unwrap_or(FunctionExp::default_constructor()),
                methods,
            };
            Ok(Exp::ClassDef(Box::new(class_exp)))
        },

        AST::PropertyAccess { exp, property } => {
            let exp = compile(exp, ctx)?;
            Ok(Exp::PropertyAccess { exp: Box::new(exp), property: property.clone() })
        },
    }
}

fn compile_function(name: Option<&str>, args: &Vec<String>, body: &AST, ctx: &mut CompilerContext) -> Result<FunctionExp, CompilerError> {
    ctx.push_frame(true);
    if name.is_some() {
        // Function is assigned to a variable in its own scope to enable recursion
        ctx.define_variable(name.unwrap().to_owned());
    }
    for arg in args {
        ctx.define_variable(arg.clone());
    }
    let result = compile(body, ctx);
    // Pops frame before eventually returning error
    ctx.pop_frame();
    Ok(FunctionExp {
        num_args: args.len(),
        external_vars: Vec::new(), // TODO: extract external variables from frame
        body: result?
    })
}

fn compile_block(ast: &AST, ctx: &mut CompilerContext, isolated: bool) -> Result<Exp, CompilerError> {
    ctx.push_frame(isolated);
    let result = compile(ast, ctx);
    // Pops frame before eventually returning error
    ctx.pop_frame();
    result
}
