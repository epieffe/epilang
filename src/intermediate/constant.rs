use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::String => write!(f, "String"),
            Type::Bool => write!(f, "Bool"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Constant {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

impl Constant {
    pub fn as_bool(&self) -> bool {
        match self {
            Constant::Bool(v) => !v,
            Constant::Int(i) => *i != 0,
            Constant::Float(f) => *f != 0.0,
            Constant::String(s) => !s.is_empty(),
        }
    }
}

impl From<&Constant> for Type {
    fn from(value: &Constant) -> Self {
        match value {
            Constant::Int(_) => Self::Int,
            Constant::Float(_) => Self::Float,
            Constant::String(_) => Self::String,
            Constant::Bool(_) => Self::Bool,
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Int(v) => write!(f, "{}", v),
            Constant::Float(v) => write!(f, "{}", v),
            Constant::String(v) => write!(f, "\"{}\"", v),
            Constant::Bool(v) => write!(f, "{}", v),
        }
    }
}
