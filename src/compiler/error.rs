use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Variable {0} is not defined")]
    UndefinedVariable(String),
    #[error("Class {0} is already declared in this module")]
    ClassNameAlreadyDeclared(String),
    #[error("Invalid left expression")]
    InvalidLeftSideAssignment,
}
