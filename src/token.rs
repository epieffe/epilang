use crate::expression::Exp;
use crate::expression::Const;
use crate::expression::Var;

pub enum Token {
    RoundBracketOpen,
    RoundBracketClosed,
    CurlyBracketOpen,
    CurlyBracketClosed,
    Operand(Operand),
    Operator(Operator),
}

pub enum Operand {
    None,
    Int(i32),
    Bool(bool),
    Str(String),
    Var(String)
}

impl Operand {
    pub fn to_exp(&self) -> Exp {
        match self {
            Operand::None => Exp::Const(Const::None),
            Operand::Int(i) => Exp::Const(Const::Integer(*i)),
            Operand::Bool(b) => Exp::Const(Const::Boolean(*b)),
            Operand::Str(s) => Exp::Const(Const::String(s.clone())),
            Operand::Var(x) => Exp::Var(Var{name: x.clone()}),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Operator {
    Assign,
    And,
    Or,
    Not,
    Eq,
    Lt,
    Gt,
    Sum,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Assign => 0,
            Operator::Mul => 1,
            Operator::Div => 1,
            Operator::Sum => 2,
            Operator::Sub => 2,
            Operator::Not => 3,
            Operator::Lt => 4,
            Operator::Gt => 4,
            Operator::Eq => 4,
            Operator::And => 5,
            Operator::Or => 6
        }
    }
}