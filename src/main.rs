mod expression;

use expression::Var;
use expression::Const;
use expression::Const::Integer;
use expression::Exp;

fn constToString(c: Const) -> String {
    match c {
        Const::Integer(i) => i.to_string(),
        Const::Boolean(b) => b.to_string(),
        Const::String(s) => format!("\"{}\"", s),
        Const::None => String::from("None")
    }
}

fn expToString(exp: Exp) -> String {
    match exp {
        Exp::Const(c) => constToString(c),
        Exp::Var(x) => x.name,
        Exp::Decl(x, e) => format!("let {};\n{}", x.name, expToString(*e)),
        Exp::Assign(x, e) => format!("{} = {}", x.name, expToString(*e)),
        Exp::Conc(e1, e2) => format!("{};\n {}", expToString(*e1), expToString(*e2)),
        Exp::Sum(e1, e2) => format!("{} + {}", expToString(*e1), expToString(*e2)),
        Exp::Sub(e1, e2) => format!("{} - {}", expToString(*e1), expToString(*e2)),
        Exp::Mul(e1, e2) => format!("{} * {}", expToString(*e1), expToString(*e2)),
        Exp::Div(e1, e2) => format!("{} / {}", expToString(*e1), expToString(*e2))
    }
}

fn main() {
    let x = Var{name: String::from("x")};

    let c = Exp::Const(Integer(3));
    let ass = Exp::Assign(x, Box::new(c));

    let p = Exp::Decl(Var{name: String::from("x")}, Box::new(ass));

    println!("{}", expToString(p));
}

