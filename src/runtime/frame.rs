use crate::ast::value::Value;
use std::collections::HashMap;
use thiserror::Error;

use super::executor::ExpressionError;

#[derive(Error, Debug)]
pub enum VariableError {
    #[error("Variable {0} is not defined")]
    UndefinedVariable(String),
}

/// Represents a stack frame.
/// It contains set of local variables and possibly a parent frame
#[derive(Debug, Default)]
pub struct Frame {
    parent: Option<Box<Frame>>,
    local_variables: HashMap<String, Value>,
}

impl Frame {
    pub fn new(parent: Box<Frame>) -> Self {
        Self {
            parent: Some(parent),
            local_variables: Default::default(),
        }
    }

    pub fn variable_value(&self, variable_name: &str) -> Result<Value, VariableError> {
        if let Some(value) = self.local_variables.get(variable_name) {
            Ok(value.clone())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.variable_value(variable_name)
        } else {
            Err(VariableError::UndefinedVariable(variable_name.to_owned()))
        }
    }

    pub fn assign_value(&mut self, variable_name: &str, value: Value) -> Result<(), ExpressionError> {
        if let Some(variable) = self.local_variables.get_mut(variable_name) {
            *variable = value;
            Ok(())
        } else if let Some(parent) = self.parent.as_mut() {
            parent.assign_value(variable_name, value)
        } else {
            Err(ExpressionError::UndefinedVariable(variable_name.to_owned()))
        }
    }

    pub fn define_variable(
        &mut self,
        variable_name: String,
        value: Value,
    ) -> () {
        if let Some(variable) = self.local_variables.get_mut(&variable_name) {
            *variable = value;
        } else {
            self.local_variables.insert(variable_name, value);
        }
    }

    pub fn take_parent(&mut self) -> Option<Box<Frame>> {
        self.parent.take()
    }
}
