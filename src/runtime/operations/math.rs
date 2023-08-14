use std::ops::{Add, Sub, Mul, Div};

use crate::intermediate::opcode::BinaryOpcode;
use crate::runtime::value::Value;
use crate::intermediate::constant::Type::{Int, Float, Bool, String};

use super::OperationError;

impl Add for &Value {
    type Output = Result<Value, OperationError>;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Value::Unit => todo!(),

            Value::Int(v1) => match other {
                Value::Unit => todo!(),
                Value::Int(v2) => Ok(Value::Int(v1 + v2)),
                Value::Float(v2) => Ok(Value::Float(*v1 as f32 + v2)),
                Value::String(v2) => Ok(Value::String(v1.to_string() + v2.as_str())),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Add, Int, Bool)),
            },

            Value::Float(v1) => match other {
                Value::Unit => todo!(),
                Value::Int(v2) => Ok(Value::Float(v1 + *v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 + v2)),
                Value::String(v2) => Ok(Value::String(v1.to_string() + v2.as_str())),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Add, Float, Bool)),
            },

            Value::String(v1) => {
                let mut result = v1.to_string();
                result.push_str(&other.to_string());
                Ok(Value::String(result))
            },

            Value::Bool(v1) => match other {
                Value::Unit => todo!(),
                Value::Int(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Add, Bool, Int)),
                Value::Float(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Add, Bool, Float)),
                Value::String(v2) => Ok(Value::String(v1.to_string() + v2.as_str())),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Add, Bool, Bool)),
            },
        }
    }
}

impl Sub for &Value {
    type Output = Result<Value, OperationError>;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Value::Unit => todo!(),

            Value::Int(v1) => match other {
                Value::Unit => todo!(),
                Value::Int(v2) => Ok(Value::Int(v1 - v2)),
                Value::Float(v2) => Ok(Value::Float(*v1 as f32 - v2)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, Int, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, Int, Bool)),
            },

            Value::Float(v1) => match other {
                Value::Unit => todo!(),
                Value::Int(v2) => Ok(Value::Float(v1 - *v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 - v2)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, Float, Bool)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, Int, Bool)),
            },

            Value::String(_) => match other {
                Value::Unit => todo!(),
                Value::Int(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, String, Int)),
                Value::Float(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, String, Float)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, String, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, String, Bool)),
            },

            Value::Bool(_) => match other {
                Value::Unit => todo!(),
                Value::Int(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, Bool, Int)),
                Value::Float(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, Bool, Float)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, Bool, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Sub, Bool, Bool)),
            },
        }
    }
}

impl Mul for &Value {
    type Output = Result<Value, OperationError>;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Value::Unit => todo!(),

            Value::Int(v1) => match other {
                Value::Unit => todo!(),
                Value::Int(v2) => Ok(Value::Int(v1 * v2)),
                Value::Float(v2) => Ok(Value::Float(*v1 as f32 * v2)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Int, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Int, Bool)),
            },
            Value::Float(v1) => match other {
                Value::Unit => todo!(),
                Value::Int(v2) => Ok(Value::Float(v1 * *v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 * v2)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Float, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Float, Bool)),
            },
            Value::String(_) => match other {
                Value::Unit => todo!(),
                Value::Int(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, String, Int)),
                Value::Float(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, String, Float)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, String, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, String, Bool)),
            },
            Value::Bool(_) => match other {
                Value::Unit => todo!(),
                Value::Int(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Bool, Int)),
                Value::Float(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Bool, Float)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Bool, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Bool, Bool)),
            },
        }
    }
}

impl Div for &Value {
    type Output = Result<Value, OperationError>;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Value::Unit => todo!(),

            Value::Int(v1) => match other {
                Value::Unit => todo!(),
                Value::Int(v2) => Ok(Value::Int(v1 / v2)),
                Value::Float(v2) => Ok(Value::Float(*v1 as f32 / v2)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, Int, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, Int, Bool)),
            },

            Value::Float(v1) => match other {
                Value::Unit => todo!(),
                Value::Int(v2) => Ok(Value::Float(v1 / *v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 / v2)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, Float, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, Float, Bool)),
            },

            Value::String(_) => match other {
                Value::Unit => todo!(),
                Value::Int(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, String, Int)),
                Value::Float(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, String, Float)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, String, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, String, Bool)),
            },

            Value::Bool(_) => match other {
                Value::Unit => todo!(),
                Value::Int(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Bool, Int)),
                Value::Float(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Bool, Float)),
                Value::String(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Bool, String)),
                Value::Bool(_) => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Bool, Bool)),
            },
        }
    }
}
