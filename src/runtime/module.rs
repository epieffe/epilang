use std::collections::HashMap;

use super::value::{Pointer, Class};
use super::pointer::Ptr;

#[derive(Default, Debug)]
pub struct Module {
    pub variables: Vec<Pointer>,
    pub classes: HashMap<usize, Ptr<Class>>,
}
