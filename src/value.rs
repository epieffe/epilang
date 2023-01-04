use std::fmt;
use std::ptr;
use std::thread;

use crate::expression::Exp;
use crate::expression::Const;

#[derive(Debug)]
pub struct Function {
    pub num_args: usize,
    pub external_values: Vec<Value>,
    pub body: Exp
}

#[derive(Copy, Clone)]
pub struct StackValue {
    value: *mut Value,
}

impl StackValue {
    pub fn from_box(val: Box<Value>) -> StackValue {
        StackValue { value: Box::into_raw(val) }
    }

    pub fn from_ptr(ptr: *mut Value) -> StackValue {
        StackValue { value: ptr }
    }

    pub fn unit() -> StackValue {
        StackValue{value: ptr::null_mut()}
    }

    pub fn read_value(&self) -> Value {
        unsafe { self.value.read() }
    }

    pub fn as_ptr(&self) -> *mut Value {
        self.value
    }

    fn as_int(&self) -> isize {
        self.value as isize
    }

    fn as_bool(&self) -> bool {
        if self.as_int() == 0 { false } else { true }
    }
}

#[derive(Debug)]
pub enum Value {
    Unit,
    Int(isize),
    Bool(bool),
    Fn(Function),
    Str(String),
}

impl Value {
    pub fn from_const(c: Const) -> Value {
        match c {
            Const::None => Value::Unit,
            Const::Integer(i) => Value::Int(isize::try_from(i).ok().unwrap()),
            Const::Boolean(b) => Value::Bool(b),
            Const::String(s) => Value::Str(s)
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "unit"),
            Value::Int(i) => write!(f, "{}", i),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Fn(func) => write!(f, "{:?}", func)
        }
    }
}
