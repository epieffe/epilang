use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

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

    pub fn variable_scope(&self, variable_name: &str) -> Option<usize> {
        if let Some(value) = self.variables.get(variable_name) {
            Some(*value)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.variable_scope(variable_name)
        } else {
            None
        }
    }

    pub fn define_variable(&mut self, variable_name: String) -> usize {
        self.variables.insert(variable_name, self.scope);
        let var_scope = self.scope;
        self.scope += 1;
        var_scope
    }
}

#[derive(Debug, Default)]
pub struct GlobalContext {
    class_counter: usize,
    classes: HashMap<String, usize>,
}

impl GlobalContext {
    pub fn class_id(&self, class_name: &str) -> Option<usize> {
        if let Some(value) = self.classes.get(class_name) {
            Some(*value)
        } else {
            None
            //Err(CompilerError::UndefinedVariable(class_name.to_owned()))
        }
    }

    pub fn define_class(&mut self, class_name: String) -> Result<usize, CompilerError> {
        match self.classes.entry(class_name) {
            // Error if a class with same name is already defined
            Occupied(o) => Err(CompilerError::ClassNameAlreadyDeclared(o.key().clone())),
            // Assign an incremental id to the class name
            Vacant(v) => {
                v.insert(self.class_counter);
                let id = self.class_counter;
                self.class_counter += 1;
                Ok(id)
            },
        }
    }
}
