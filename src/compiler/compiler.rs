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

pub struct CompilerContext {
    pub current_scope: usize,
    pub variable_map: HashMap<String, usize>,
}

pub fn compile(ast: &AST, context: &mut CompilerContext) -> Result<Exp, CompilerError> {

    match ast {
        AST::Constant(value) => {
            Ok(Exp::Constant { value: value.clone() })
        },

        AST::Identifier(name) => {
            let scope = context.variable_map.get(name).ok_or(CompilerError::UndefinedVariable(name.clone()))?;
            Ok(Exp::Variable { scope: *scope })
        },

        AST::Concatenation { left, right } => {
            let exp1 = compile(left, context)?;
            let exp2 = compile(right, context)?;
            Ok(Exp::Concatenation { first: Box::new(exp1), second: Box::new(exp2) })
        },

        AST::BinaryOp(_, _, _) => todo!(),

        AST::UnaryOp(_, _) => todo!(),

        AST::Definition(name) => todo!(),

        AST::Assignment(_, _) => todo!(),

        AST::Block(_) => todo!(),

        AST::Condition { exp, then_block, else_block } => todo!(),
    }
}
