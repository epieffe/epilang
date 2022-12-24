pub enum Exp {
    Const(Const),// Eg: 1, False, None, "Hello"
    Var(Var),// Eg: x, y, z
    Decl(Var, Box<Exp>),// Eg: let x
    Assign(Var, Box<Exp>),// Eg: x = 3
    Conc(Box<Exp>, Box<Exp>),// Eg: x; y
    Sum(Box<Exp>, Box<Exp>),// Eg: x + y
    Sub(Box<Exp>, Box<Exp>),// Eg: x - y
    Mul(Box<Exp>, Box<Exp>),// Eg: x * y
    Div(Box<Exp>, Box<Exp>),// Eg: x / y
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
