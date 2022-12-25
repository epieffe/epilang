pub enum Exp {
    // Eg: 1, False, None, "Hello"
    Const(Const),
    // Eg: x, y, z
    Var(Var),
    // Eg: let x = y
    Decl(Var, Box<Exp>, Box<Exp>),
    // Eg: x = 3
    Assign(Var, Box<Exp>),
    // Eg: x; y
    Seq(Box<Exp>, Box<Exp>),
    // Eg: x + y
    Sum(Box<Exp>, Box<Exp>),
    // Eg: x - y
    Sub(Box<Exp>, Box<Exp>),
    // Eg: x * y
    Mul(Box<Exp>, Box<Exp>),
    // Eg: x / y
    Div(Box<Exp>, Box<Exp>),
    // Eg: x < y
    Lt(Box<Exp>, Box<Exp>),
    // Eg: x > y
    Gt(Box<Exp>, Box<Exp>),
    // Eg: x == y
    Eq(Box<Exp>, Box<Exp>),
    // Eg: x && y
    And(Box<Exp>, Box<Exp>),
    // Eg: x || y
    Or(Box<Exp>, Box<Exp>),
    // Eg: !x
    Not(Box<Exp>),
    // If then else. Eg: if e {e1} else {e2}
    IfThenElse(Box<Exp>, Box<Exp>, Box<Exp>)
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
