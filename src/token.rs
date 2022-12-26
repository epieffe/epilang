use crate::expression::Exp;
use crate::expression::Const;
use crate::expression::Var;

pub enum Token {
    Operand(Operand),
    Operator(Operator),
    If,
    Else,
    Let,
    RoundBracketOpen,
    RoundBracketClosed,
    CurlyBracketOpen,
    CurlyBracketClosed,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Operator(Operator::Assign), Token::Operator(Operator::Assign)) => true,
            (Token::Operator(Operator::Seq), Token::Operator(Operator::Seq)) => true,
            (Token::CurlyBracketOpen, Token::CurlyBracketOpen) => true,
            (Token::CurlyBracketClosed, Token::CurlyBracketClosed) => true,
            (Token::If, Token::If) => true,
            (Token::Else, Token::Else) => true,
            (Token::RoundBracketOpen, Token::RoundBracketOpen) => true,
            (Token::RoundBracketClosed, Token::RoundBracketClosed) => true,
            (Token::Let, Token::Let) => true,
            (Token::Operator(Operator::Eq), Token::Operator(Operator::Eq)) => true,
            _ => false
        }
    }
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
    Seq,
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
    Div
}

impl Operator {
    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Mul => 1,
            Operator::Div => 1,
            Operator::Sum => 2,
            Operator::Sub => 2,
            Operator::Not => 3,
            Operator::Lt => 4,
            Operator::Gt => 4,
            Operator::Eq => 4,
            Operator::And => 5,
            Operator::Or => 6,
            Operator::Assign => 7,
            Operator::Seq => 8,
        }
    }
}
