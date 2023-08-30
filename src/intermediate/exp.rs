use std::collections::HashMap;

use super::constant::Constant;
use super::opcode::BinaryOpcode;
use super::opcode::UnaryOpcode;

#[derive(Clone, Debug)]
pub enum Exp {
    Constant { value: Constant },
    Variable { scope: usize },
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
    BuiltInFunction(BuiltInFunction),
    Closure(Box<FunctionExp>),
    FunctionCall { fun: Box<Exp>, args: Vec<Exp> },
    ClassDef(Box<ClassExp>),
    Class{id: usize},
    PropertyAccess { exp: Box<Exp>, property: String },
}

impl Default for Exp {
    fn default() -> Self {
        Exp::Constant { value: Constant::Unit }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionExp {
    pub num_args: usize,
    pub external_vars: Vec<usize>,
    pub body: Exp,
}

impl FunctionExp {
    pub fn default_constructor() -> FunctionExp {
        FunctionExp {
            num_args: 1,
            external_vars: Vec::default(),
            body: Exp::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassExp {
    pub id: usize,
    pub name: String,
    pub fields: Vec<String>,
    pub constructor: FunctionExp,
    pub methods: HashMap<String, FunctionExp>,
}

#[derive(Copy, Clone, Debug)]
pub enum BuiltInFunction {
    Print,
    Println,
    Input,
}
