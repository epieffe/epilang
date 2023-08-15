use thiserror::Error;

use crate::compiler::compiler::CompilerError;
use crate::runtime::executor::ExpressionError;

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("SyntaxError")]
    SyntaxError,
    #[error("CompilerError: {0}")]
    CompilerError(CompilerError),
    #[error("RuntimeError: {0}")]
    RuntimeError(ExpressionError),
}
