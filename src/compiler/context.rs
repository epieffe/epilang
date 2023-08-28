use std::collections::HashMap;

use super::error::CompilerError;

#[derive(Debug)]
struct Frame {
    pub var_scope: usize,
    // Maps each variable name to its scope
    pub variables: HashMap<String, usize>,
    // Maps each class name with its id
    pub classes: HashMap<String, usize>,
    pub isolated: bool,
}

#[derive(Debug)]
pub struct CompilerContext {
    frames: Vec<Frame>,
    class_count: usize,
}

impl CompilerContext {
    pub fn new() -> CompilerContext {
        let root_frame = Frame {
            var_scope: 0,
            variables: HashMap::new(),
            classes: HashMap::new(),
            isolated: false,
        };
        let frames = vec![root_frame];
        CompilerContext { frames, class_count: 0 }
    }

    pub fn push_frame(&mut self, isolated: bool) {
        let last = self.frames.last().unwrap();
        let new_frame = Frame {
            var_scope: if isolated { 0 } else { last.var_scope },
            variables: HashMap::new(),
            classes: HashMap::new(),
            isolated
        };
        self.frames.push(new_frame)
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn variable_scope(&self, variable_name: &str) -> Option<usize> {
        for frame in self.frames.iter().rev() {
            if let Some(value) = frame.variables.get(variable_name) {
                return Some(*value)
            }
            if frame.isolated { break }

        }
        None
    }

    pub fn define_variable(&mut self, variable_name: String) -> usize {
        let frame = self.frames.last_mut().unwrap();
        frame.variables.insert(variable_name, frame.var_scope);
        let var_scope = frame.var_scope;
        frame.var_scope += 1;
        var_scope
    }

    pub fn class_id(&self, class_name: &str) -> Option<usize> {
        for frame in self.frames.iter().rev() {
            if let Some(value) = frame.classes.get(class_name) {
                return Some(*value)
            }
        }
        None
    }

    pub fn define_class(&mut self, class_name: String) -> Result<usize, CompilerError> {
        let frame = self.frames.last_mut().unwrap();
        let class_id = self.class_count;
        frame.classes.insert(class_name, class_id);
        self.class_count += 1;
        Ok(class_id)
    }
}
