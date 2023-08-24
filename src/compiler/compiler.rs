use crate::intermediate::exp::Exp;

use super::ast::AST;
use super::error::CompilerError;
use super::frame::Frame;

pub fn compile(ast: &AST, frame: &mut Frame) -> Result<Exp, CompilerError> {

    match ast {
        AST::Constant(value) => {
            Ok(Exp::Constant { value: value.clone() })
        },

        AST::Identifier(name) => {
            let scope = frame.variable_scope(name)?;
            Ok(Exp::Variable { scope })
        },

        AST::Concatenation { left, right } => {
            let exp1 = compile(left, frame)?;
            let exp2 = compile(right, frame)?;
            Ok(Exp::Concatenation { first: Box::new(exp1), second: Box::new(exp2) })
        },

        AST::BinaryOp(arg1, op, arg2) => {
            let exp1 = compile(arg1, frame)?;
            let exp2 = compile(arg2, frame)?;
            Ok(Exp::BinaryOp { op: *op, arg1: Box::new(exp1), arg2: Box::new(exp2) })
        },

        AST::UnaryOp(op, arg) => {
            let exp = compile(arg, frame)?;
            Ok(Exp::UnaryOp { op: *op, arg: Box::new(exp) })
        },

        AST::Definition(name) => {
            let scope = frame.define_variable(name.clone());
            Ok(Exp::Let { scope })
        },

        AST::Assignment(left, right) => {
            let left_exp = compile(left, frame)?;
            let right_exp = compile(right, frame)?;
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
                _ => Err(CompilerError::InvalidLeftSideAssignment(left.to_string()))
            }
        },

        AST::Block(exp) => {
            let mut sub_frame = Frame::new(frame);
            let exp = compile(exp, &mut sub_frame)?;
            Ok(Exp::Block { exp: Box::new(exp) })
        },

        AST::Condition { exp, then_block, else_block } => {
            let exp = compile(exp, frame)?;
            // then block
            let mut then_frame = Frame::new(frame);
            let then_block = compile(then_block, &mut then_frame)?;
            // else block
            let mut else_frame = Frame::new(frame);
            let else_block = compile(else_block, &mut else_frame)?;
            Ok(Exp::Condition {
                exp: Box::new(exp),
                then_block: Box::new(then_block),
                else_block: Box::new(else_block)
            })
        },

        AST::While {guard, exp } => {
            let guard = compile(guard, frame)?;
            let exp = compile(exp, frame)?;
            Ok(Exp::While { guard: Box::new(guard), exp: Box::new(exp) })
        },

        AST::List { elements } => {
            let mut list = Vec::with_capacity(elements.len());
            for element in elements {
                list.push(compile(element, frame)?)
            }
            Ok(Exp::List { elements: list })
        },

        AST::Subscript { element, index } => {
            let e = compile(element, frame)?;
            let i = compile(index, frame)?;
            Ok(Exp::Subscript { element: Box::new(e), index: Box::new(i) })
        }

        AST::Closure { args, exp } => {
            let mut function_frame: Frame = Default::default();
            for arg in args {
                function_frame.define_variable(arg.clone());
            }
            let exp = compile(exp, &mut function_frame)?;
            Ok(Exp::Closure { num_args: args.len(), exp: Box::new(exp) })
        },

        AST::FunctionCall { fun, args } => {
            let fun_exp = compile(fun, frame)?;
            let mut args_exps = Vec::new();
            for arg in args {
                let arg_exp = compile(arg, frame)?;
                args_exps.push(arg_exp);
            };
            Ok(Exp::FunctionCall { fun: Box::new(fun_exp), args: args_exps })
        }
    }
}
