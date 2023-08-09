use crate::intermediate::constant::Constant;
use crate::operations::OperationError;
use std::ops::{Add, Div, Mul, Sub};

impl Add for Constant {
    type Output = Result<Constant, OperationError>;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Constant::Int(v1) => match other {
                Constant::Int(v2) => Ok(Constant::Int(v1 + v2)),
                Constant::Float(v2) => Ok(Constant::Float(v1 as f32 + v2)),
                Constant::String(v2) => Ok(Constant::String(v1.to_string() + v2.as_str())),
                Constant::Bool(_) => error!(Int, Add, Bool),
            },
            Constant::Float(v1) => match other {
                Constant::Int(v2) => Ok(Constant::Float(v1 + v2 as f32)),
                Constant::Float(v2) => Ok(Constant::Float(v1 + v2)),
                Constant::String(v2) => Ok(Constant::String(v1.to_string() + v2.as_str())),
                Constant::Bool(_) => error!(Float, Add, Bool),
            },
            Constant::String(v1) => match other {
                Constant::Int(v2) => Ok(Constant::String(v1 + v2.to_string().as_str())),
                Constant::Float(v2) => Ok(Constant::String(v1 + v2.to_string().as_str())),
                Constant::String(v2) => Ok(Constant::String(v1 + v2.as_str())),
                Constant::Bool(v2) => Ok(Constant::String(v1 + v2.to_string().as_str())),
            },
            Constant::Bool(v1) => match other {
                Constant::Int(_) => error!(Bool, Add, Int),
                Constant::Float(_) => error!(Bool, Add, Int),
                Constant::String(v2) => Ok(Constant::String(v1.to_string() + v2.as_str())),
                Constant::Bool(_) => error!(Bool, Add, Bool),
            },
        }
    }
}

impl Sub for Constant {
    type Output = Result<Constant, OperationError>;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Constant::Int(v1) => match other {
                Constant::Int(v2) => Ok(Constant::Int(v1 - v2)),
                Constant::Float(v2) => Ok(Constant::Float(v1 as f32 - v2)),
                Constant::String(_) => error!(Int, Sub, String),
                Constant::Bool(_) => error!(Int, Sub, Bool),
            },
            Constant::Float(v1) => match other {
                Constant::Int(v2) => Ok(Constant::Float(v1 - v2 as f32)),
                Constant::Float(v2) => Ok(Constant::Float(v1 - v2)),
                Constant::String(_) => error!(Float, Sub, String),
                Constant::Bool(_) => error!(Float, Sub, Bool),
            },
            Constant::String(_) => match other {
                Constant::Int(_) => error!(String, Sub, Int),
                Constant::Float(_) => error!(String, Sub, Float),
                Constant::String(_) => error!(String, Sub, String),
                Constant::Bool(_) => error!(String, Sub, Bool),
            },
            Constant::Bool(_) => error_other!(Bool, Sub, other),
        }
    }
}

impl Mul for Constant {
    type Output = Result<Constant, OperationError>;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Constant::Int(v1) => match other {
                Constant::Int(v2) => Ok(Constant::Int(v1 * v2)),
                Constant::Float(v2) => Ok(Constant::Float(v1 as f32 * v2)),
                Constant::String(_) => error!(Int, Mul, String),
                Constant::Bool(_) => error!(Int, Mul, Bool),
            },
            Constant::Float(v1) => match other {
                Constant::Int(v2) => Ok(Constant::Float(v1 * v2 as f32)),
                Constant::Float(v2) => Ok(Constant::Float(v1 * v2)),
                Constant::String(_) => error!(Float, Mul, String),
                Constant::Bool(_) => error!(Float, Mul, Bool),
            },
            Constant::String(_) => match other {
                Constant::Int(_) => error!(String, Mul, Int),
                Constant::Float(_) => error!(String, Mul, Float),
                Constant::String(_) => error!(String, Mul, String),
                Constant::Bool(_) => error!(String, Div, Bool),
            },
            Constant::Bool(_) => error_other!(Bool, Mul, other),
        }
    }
}

impl Div for Constant {
    type Output = Result<Constant, OperationError>;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Constant::Int(v1) => match other {
                Constant::Int(v2) => Ok(Constant::Int(v1 / v2)),
                Constant::Float(v2) => Ok(Constant::Float(v1 as f32 / v2)),
                Constant::String(_) => error!(Int, Div, String),
                Constant::Bool(_) => error!(Int, Mul, Bool),
            },
            Constant::Float(v1) => match other {
                Constant::Int(v2) => Ok(Constant::Float(v1 / v2 as f32)),
                Constant::Float(v2) => Ok(Constant::Float(v1 / v2)),
                Constant::String(_) => error!(Float, Div, String),
                Constant::Bool(_) => error!(Float, Div, Bool),
            },
            Constant::String(_) => match other {
                Constant::Int(_) => error!(String, Div, Int),
                Constant::Float(_) => error!(String, Div, Float),
                Constant::String(_) => error!(String, Div, String),
                Constant::Bool(_) => error!(String, Div, Bool),
            },
            Constant::Bool(_) => error_other!(Bool, Div, other),
        }
    }
}
