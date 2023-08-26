use std::collections::HashMap;

use super::constant::Constant;
use super::opcode::BinaryOpcode;
use super::opcode::UnaryOpcode;

#[derive(Clone, Debug)]
pub enum Exp {
    Constant { value: Constant },
    Variable{ scope: usize },
    Concatenation { first: Box<Exp>, second: Box<Exp> },
    BinaryOp { op: BinaryOpcode, arg1: Box<Exp>, arg2: Box<Exp> },
    UnaryOp { op: UnaryOpcode, arg: Box<Exp> },
    Let {scope: usize },
    Assignment { left: Box<Exp>, right: Box<Exp> },
    Block { exp: Box<Exp> },
    Condition { exp: Box<Exp>, then_block: Box<Exp>, else_block: Box<Exp> },
    While { guard: Box<Exp>, exp: Box<Exp> },
    List { elements: Vec<Exp> },
    Subscript { element: Box<Exp>, index: Box<Exp> },
    Function(Box<FunctionExp>),
    Closure(Box<FunctionExp>),
    FunctionCall { fun: Box<Exp>, args: Vec<Exp> },
    Class(Box<ClassExp>),
}

#[derive(Clone, Debug)]
pub struct FunctionExp {
    pub num_args: usize,
    pub external_vars: Vec<usize>,
    pub body: Exp,
}

#[derive(Clone, Debug)]
pub struct ClassExp {
    pub id: usize,
    pub name: String,
    pub fields: Vec<String>,
    pub methods: HashMap<String, FunctionExp>,
}
