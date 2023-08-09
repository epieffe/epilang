use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum BinaryOpcode {
    Mul,
    Div,
    Add,
    Sub,
    Conj,
    Disj,
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Lower,
    LowerEquals,
}

impl Display for BinaryOpcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOpcode::Mul => write!(f, "*"),
            BinaryOpcode::Div => write!(f, "/"),
            BinaryOpcode::Add => write!(f, "+"),
            BinaryOpcode::Sub => write!(f, "-"),
            BinaryOpcode::Conj => write!(f, "&&"),
            BinaryOpcode::Disj => write!(f, "||"),
            BinaryOpcode::Equals => write!(f, "=="),
            BinaryOpcode::NotEquals => write!(f, "!="),
            BinaryOpcode::Greater => write!(f, ">"),
            BinaryOpcode::GreaterEquals => write!(f, ">="),
            BinaryOpcode::Lower => write!(f, "<"),
            BinaryOpcode::LowerEquals => write!(f, "<="),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum UnaryOpcode {
    Not,
}

impl Display for UnaryOpcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOpcode::Not => write!(f, "!"),
        }
    }
}
