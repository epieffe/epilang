use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Unit,
    Int,
    Float,
    String,
    Bool,
    List,
    Function,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => write!(f, "Unit"),
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::String => write!(f, "String"),
            Type::Bool => write!(f, "Bool"),
            Type::List => write!(f, "List"),
            Type::Function => write!(f, "Function"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Constant {
    Unit,
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Unit => write!(f, "unit"),
            Constant::Int(v) => write!(f, "{}", v),
            Constant::Float(v) => write!(f, "{}", v),
            Constant::String(v) => write!(f, "\"{}\"", v),
            Constant::Bool(v) => write!(f, "{}", v),
        }
    }
}
