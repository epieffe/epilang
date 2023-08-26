use super::constant::Constant;
use super::opcode::BinaryOpcode;
use super::opcode::UnaryOpcode;

#[derive(PartialEq, Clone, Debug)]
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
    Function { num_args: usize, external_vars: Vec<usize>, exp: Box<Exp> },
    Closure { num_args: usize, external_vars: Vec<usize>, exp: Box<Exp> },
    FunctionCall { fun: Box<Exp>, args: Vec<Exp> },
}
