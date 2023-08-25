use std::ops::Not;
use std::cmp::Ordering;

use crate::runtime::value::Value;

impl Not for &Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        Value::Bool(!self.as_bool())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Unit, Value::Unit) => true,
            (Value::Int(i1), Value::Int(i2)) => i1 == i2,
            (Value::Float(f1), Value::Float(f2)) => f1 == f2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (v1, v2) => std::ptr::eq(v1, v2),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Unit, Value::Unit) => Some(Ordering::Equal),
            (Value::Int(i1), Value::Int(i2)) => Some(i1.cmp(i2)),
            (Value::Float(f1), Value::Float(f2)) => f1.partial_cmp(f2),
            (Value::Bool(b1), Value::Bool(b2)) => Some(b1.cmp(b2)),
            (Value::String(s1), Value::String(s2)) => Some(s1.cmp(s2)),
            (v1, v2) => if v1 == v2 { Some(Ordering::Equal) } else { None },
        }
    }
}
