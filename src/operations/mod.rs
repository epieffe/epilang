use crate::intermediate::opcode::BinaryOpcode;
use crate::intermediate::opcode::UnaryOpcode;
use crate::intermediate::constant::Type;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Operation {0} {1} {2} is not defined")]
    IncompatibleTypes(Type, BinaryOpcode, Type),
    #[error("Operation {0} {1} is not defined")]
    IncompatibleType(UnaryOpcode, Type),
}

macro_rules! error {
    ($type_1:ident, $op:ident, $type_2:ident) => {
        Err(OperationError::IncompatibleTypes(
            crate::intermediate::constant::Type::$type_1,
            crate::intermediate::opcode::BinaryOpcode::$op,
            crate::intermediate::constant::Type::$type_2,
        ))
    };
}

macro_rules! error_other {
    ($type_1:ident, $op:ident, $other:ident) => {
        Err(OperationError::IncompatibleTypes(
            crate::intermediate::constant::Type::$type_1,
            crate::intermediate::opcode::BinaryOpcode::$op,
            (&$other).into(),
        ))
    };
}

mod logical;
mod math;

pub use logical::*;
