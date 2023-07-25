use crate::ast::value::Value;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum Expr {
    Constant(Value),
    Identifier(String),
    Concatenation { left: Box<Expr>, right: Box<Expr> },
    BinaryOp(Box<Expr>, BinaryOpcode, Box<Expr>),
    UnaryOp(UnaryOpcode, Box<Expr>),
    Definition(String),
    Assignment(Box<Expr>, Box<Expr>),
    Block(Box<Expr>),
    Condition { exp: Box<Expr>, then_block: Box<Expr>, else_block: Box<Expr> },
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Constant(v) => write!(f, "{}", v),
            Expr::Identifier(id) => write!(f, "{}", id),
            Expr::Concatenation { left, right } => write!(f, "({}; {})", left, right),
            Expr::BinaryOp(e1, op, e2) => write!(f, "({} {} {})", e1, op, e2),
            Expr::UnaryOp(op, e) => write!(f, "({}{})", op, e),
            Expr::Definition(var) => write!(f, "(let {})", var),
            Expr::Assignment(var, e) => write!(f, "({} = {})", var, e),
            Expr::Block(e) => write!(f, "({{{}}})", e),
            Expr::Condition{ exp, then_block, else_block } => {
                write!(f, "(if {} {{{}}} else {{{}}})", exp, then_block, else_block)
            },
        }
    }
}

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

#[cfg(test)]
mod test {
    use crate::ast::lr_lang;
    use rstest::*;

    #[rstest]
    #[case("1 + 2 * 3 - 4", "((1 + (2 * 3)) - 4)")]
    #[case("(1 + 2) * (3 - 4)", "((1 + 2) * (3 - 4))")]
    #[case("true || false", "(true || false)")]
    #[case(
        "2 > 3 && 3 >= 4 || 5 < 6 && 7 <= 8 || 9 == 10 && a != !b",
        "((((2 > 3) && (3 >= 4)) || ((5 < 6) && (7 <= 8))) || ((9 == 10) && (a != (!b))))"
    )]
    #[case(
        "5 + 5 >= 100 * 12 + 3 - 1 || abc != xyz",
        "(((5 + 5) >= (((100 * 12) + 3) - 1)) || (abc != xyz))"
    )]
    fn test_expression_parser(#[case] expression: &str, #[case] expected: &str) {
        let parsed = lr_lang::ExprParser::new()
            .parse(expression)
            .expect("Unable to parse expression");
        assert_eq!(expected, parsed.to_string())
    }
}
