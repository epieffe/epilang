pub enum Exp {
    Const(Const),// Eg: 1, False, None, "Hello"
    Var(Var),// Eg: x, y, z
    Decl(Var, Box<Exp>),// Eg: let x
    Assign(Var, Box<Exp>),// Eg: x = 3
    Seq(Box<Exp>, Box<Exp>),// Eg: x; y
    Sum(Box<Exp>, Box<Exp>),// Eg: x + y
    Sub(Box<Exp>, Box<Exp>),// Eg: x - y
    Mul(Box<Exp>, Box<Exp>),// Eg: x * y
    Div(Box<Exp>, Box<Exp>),// Eg: x / y
    Lt(Box<Exp>, Box<Exp>),// Eg: x < y
    Gt(Box<Exp>, Box<Exp>),// Eg: x > y
    Eq(Box<Exp>, Box<Exp>),// Eg: x == y
    And(Box<Exp>, Box<Exp>),// Eg: x && y
    Or(Box<Exp>, Box<Exp>),// Eg: x || y
    Not(Box<Exp>),// Eg: !x
}

pub struct Var {
    pub name: String
}

pub enum Const {
    Integer(i32),
    Boolean(bool),
    String(String),
    None
}
