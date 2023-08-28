use crate::intermediate::constant::Constant;
use crate::intermediate::opcode::BinaryOpcode;
use crate::intermediate::opcode::UnaryOpcode;

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
    Function(Box<FunctionAST>),
    Closure { args: Vec<String>, exp: Box<AST> },
    FunctionCall { fun: Box<AST>, args: Vec<AST> },
    Class(Box<ClassAST>),
    PropertyAccess { exp: Box<AST>, property: String },
}

pub struct FunctionAST {
    pub name: String,
    pub args: Vec<String>,
    pub body: AST,
}

pub struct ClassAST {
    pub name: String,
    pub fields: Vec<String>,
    pub constructor: Option<FunctionAST>,
    pub methods: Vec<FunctionAST>,
}
