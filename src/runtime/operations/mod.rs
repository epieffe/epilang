use crate::intermediate::opcode::BinaryOpcode;
use crate::intermediate::constant::Type;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Unsupported operand types for {0}: {1} {2}")]
    IncompatibleTypes(BinaryOpcode, Type, Type),
}
mod math;
mod logical;
