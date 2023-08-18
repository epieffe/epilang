use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Variable {0} is not defined")]
    UndefinedVariable(String),
    #[error("Invalid left expression: {0}")]
    InvalidLeftSideAssignment(String),
}
