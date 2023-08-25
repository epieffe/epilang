use crate::intermediate::constant::Constant;
use crate::intermediate::opcode::BinaryOpcode;
use crate::intermediate::opcode::UnaryOpcode;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum AST {
    Constant(Constant),
    Identifier(String),
    Concatenation { left: Box<AST>, right: Box<AST> },
    BinaryOp(Box<AST>, BinaryOpcode, Box<AST>),
    UnaryOp(UnaryOpcode, Box<AST>),
    Definition(String),
    Assignment(Box<AST>, Box<AST>),
    Block(Box<AST>),
    Condition { exp: Box<AST>, then_block: Box<AST>, else_block: Box<AST> },
    While { guard: Box<AST>, exp: Box<AST> },
    List { elements: Vec<AST> },
    Subscript { element: Box<AST>, index: Box<AST> },
    Closure { args: Vec<String>, exp: Box<AST> },
    FunctionCall { fun: Box<AST>, args: Vec<AST> },
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AST::Constant(v) => write!(f, "{}", v),
            AST::Identifier(id) => write!(f, "{}", id),
            AST::Concatenation { left, right } => write!(f, "({}; {})", left, right),
            AST::BinaryOp(e1, op, e2) => write!(f, "({} {} {})", e1, op, e2),
            AST::UnaryOp(op, e) => write!(f, "({}{})", op, e),
            AST::Definition(var) => write!(f, "(let {})", var),
            AST::Assignment(var, e) => write!(f, "({} = {})", var, e),
            AST::Block(e) => write!(f, "({{{}}})", e),
            AST::Condition{ exp, then_block, else_block } => {
                write!(f, "(if {} {{{}}} else {{{}}})", exp, then_block, else_block)
            },
            AST::While { guard, exp } => {
                write!(f, "while {} {{{}}}", guard, exp)
            },
            AST::List { elements } => {
                write!(f, "[")?;
                for element in elements {
                    write!(f, "{}, ", element)?;
                }
                write!(f, "]")
            },
            AST::Subscript { element, index } => {
                write!(f, "{}[{}]", element, index)
            },
            AST::Closure { args, exp } => {
                write!(f, "fn(")?;
                for arg in args {
                    write!(f, "{}, ", arg)?;
                }
                write!(f, ") {{{}}}", exp)
            },
            AST::FunctionCall { fun, args } => {
                write!(f, "{}(", fun)?;
                for arg in args {
                    write!(f, "{}, ", arg)?;
                }
                write!(f, ")")
            },
        }
    }
}
