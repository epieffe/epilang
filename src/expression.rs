use std::fmt;

pub enum Exp {
    // Eg: 1, False, None, "Hello"
    Const(Const),
    // Eg: x, y, z
    Var(Var),
    // Eg: let x = exp1; exp2
    Decl(Var, Box<Exp>, Box<Exp>),
    // Eg: x = exp
    Assign(Var, Box<Exp>),
    // Eg: exp1; exp2
    Seq(Box<Exp>, Box<Exp>),
    // Eg: exp1 + exp2
    Sum(Box<Exp>, Box<Exp>),
    // Eg: exp1 - exp2
    Sub(Box<Exp>, Box<Exp>),
    // Eg: exp1 * exp2
    Mul(Box<Exp>, Box<Exp>),
    // Eg: exp1 / exp2
    Div(Box<Exp>, Box<Exp>),
    // Eg: exp1 < exp2
    Lt(Box<Exp>, Box<Exp>),
    // Eg: exp1 > exp2
    Gt(Box<Exp>, Box<Exp>),
    // Eg: exp1 == exp2
    Eq(Box<Exp>, Box<Exp>),
    // Eg: exp1 != exp2
    Neq(Box<Exp>, Box<Exp>),
    // Eg: exp1 && exp2
    And(Box<Exp>, Box<Exp>),
    // Eg: exp1 || exp2
    Or(Box<Exp>, Box<Exp>),
    // Eg: !exp
    Not(Box<Exp>),
    // If then else. Eg: if exp {exp1} else {exp2}
    IfThenElse(Box<Exp>, Box<Exp>, Box<Exp>)
}

pub struct Var {
    pub name: String,
    pub scope: usize
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Const {
    Integer(i32),
    Boolean(bool),
    String(String),
    None
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Const::Integer(i) => write!(f, "{}", i),
            Const::Boolean(b) => write!(f, "{}", b),
            Const::String(s) => write!(f, "{}", s),
            Const::None => write!(f, "unit")
        }
    }
}

impl PartialEq for Const {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Const::Integer(i1), Const::Integer(i2)) => i1 == i2,
            (Const::Boolean(b1), Const::Boolean(b2)) => b1 == b2,
            (Const::String(s1), Const::String(s2)) => s1 == s2,
            (Const::None, Const::None) => true,
            _ => false
        }
    }
}
