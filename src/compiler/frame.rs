use std::collections::HashMap;

use super::error::CompilerError;

#[derive(Debug, Default)]
pub struct Frame<'a> {
    pub scope: usize,
    // Maps each variable name to its scope
    variables: HashMap<String, usize>,
    parent: Option<&'a Frame<'a>>,
}

impl <'a> Frame<'a> {
    pub fn new(parent: &'a Frame) -> Self {
        Self {
            scope: parent.scope,
            variables: Default::default(),
            parent: Some(parent),
        }
    }

    pub fn variable_scope(&self, variable_name: &str) -> Result<usize, CompilerError> {
        if let Some(value) = self.variables.get(variable_name) {
            Ok(*value)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.variable_scope(variable_name)
        } else {
            Err(CompilerError::UndefinedVariable(variable_name.to_owned()))
        }
    }

    pub fn define_variable(&mut self, variable_name: String) -> usize {
        self.variables.insert(variable_name, self.scope);
        let var_scope = self.scope;
        self.scope += 1;
        var_scope
    }
}
