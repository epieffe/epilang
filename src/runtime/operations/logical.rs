use crate::runtime::value::Value;

use std::ops::Not;

impl Not for &Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        Value::Bool(!self.as_bool())
    }
}
