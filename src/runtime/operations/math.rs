use std::ops::{Add, Sub, Mul, Div};

use crate::intermediate::opcode::BinaryOpcode;
use crate::runtime::value::Value;
use crate::intermediate::constant::Type::{Int, Float};

use super::OperationError;

impl Add for &Value {
    type Output = Result<Value, OperationError>;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Value::Int(v1) => match other {
                Value::Int(v2) => Ok(Value::Int(v1 + v2)),
                Value::Float(v2) => Ok(Value::Float(*v1 as f32 + v2)),
                Value::String(v2) => Ok(Value::String(v1.to_string() + v2.as_str())),
                v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Add, Int, v.get_type())),
            },

            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 + *v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 + v2)),
                Value::String(v2) => Ok(Value::String(v1.to_string() + v2.as_str())),
                v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Float, v.get_type())),
            },

            Value::String(v1) => {
                let mut result = v1.to_string();
                result.push_str(&other.to_string());
                Ok(Value::String(result))
            },

            v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Add, v.get_type(), other.get_type())),
        }
    }
}

impl Sub for &Value {
    type Output = Result<Value, OperationError>;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Value::Int(v1) => match other {
                Value::Int(v2) => Ok(Value::Int(v1 - v2)),
                Value::Float(v2) => Ok(Value::Float(*v1 as f32 - v2)),
                v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, Int, v.get_type())),
            },

            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 - *v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 - v2)),
                v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, Int, v.get_type())),
            },

            v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, v.get_type(), other.get_type())),
        }
    }
}

impl Mul for &Value {
    type Output = Result<Value, OperationError>;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Value::Int(v1) => match other {
                Value::Int(v2) => Ok(Value::Int(v1 * v2)),
                Value::Float(v2) => Ok(Value::Float(*v1 as f32 * v2)),
                v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Int, v.get_type())),
            },

            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 * *v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 * v2)),
                v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, Float, v.get_type())),
            },

            v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, v.get_type(), other.get_type())),
        }
    }
}

impl Div for &Value {
    type Output = Result<Value, OperationError>;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Value::Int(v1) => match other {
                Value::Int(v2) => Ok(Value::Int(v1 / v2)),
                Value::Float(v2) => Ok(Value::Float(*v1 as f32 / v2)),
                v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, Int, v.get_type())),
            },

            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 / *v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 / v2)),
                v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Div, Float, v.get_type())),
            },

            v => Err(OperationError::IncompatibleTypes(BinaryOpcode::Mul, v.get_type(), other.get_type())),
        }
    }
}
