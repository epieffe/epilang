use std::collections::HashMap;

use super::value::{Value, Class};
use super::pointer::Ptr;

#[derive(Default, Debug)]
pub struct Module {
    pub variables: Vec<Ptr<Value>>,
    pub classes: HashMap<usize, Ptr<Class>>,
}
