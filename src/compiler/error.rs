use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Unknown identifier '{0}'")]
    UnknownIdentifier(String),
    #[error("Class {0} is already declared in this module")]
    ClassNameAlreadyDeclared(String),
    #[error("Invalid left expression")]
    InvalidLeftSideAssignment,
}
