use std::fmt;
use std::ptr;
use std::thread;

use crate::expression::Exp;
use crate::expression::Const;

#[derive(Debug)]
pub struct Function {
    pub num_args: usize,
    pub external_values: Vec<Value>,
    pub body: Box<Exp>
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

    pub fn is_unit(&self) -> bool {
        self.value.is_null()
    }

    pub fn as_ref(&self) -> &Value {
        unsafe{ &*self.value }
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
    pub fn from_const(c: &Const) -> Value {
        match c {
            Const::None => Value::Unit,
            Const::Integer(i) => Value::Int(isize::try_from(*i).ok().unwrap()),
            Const::Boolean(b) => Value::Bool(*b),
            Const::String(s) => Value::Str(s.clone())
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Unit => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Str(s) => s == "",
            Value::Fn(_) => true
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

///////////

pub enum V {
    Val(Value),
    Ptr(StackValue)
}

impl V {
    pub fn as_bool(&self) -> bool {
        match self {
            V::Ptr(ptr) => if ptr.is_unit() {false} else {ptr.as_ref().as_bool()},
            V::Val(value) => value.as_bool()
        }
    }

    pub fn as_ref(&self) -> &Value {
        match self {
            V::Ptr(ptr) => ptr.as_ref(),
            V::Val(value) => value
        }
    }
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            V::Ptr(ptr) => if ptr.is_unit() {write!(f, "unit")}  else {write!(f, "{}", ptr.as_ref())},
            V::Val(value) => write!(f, "{}", value)
        }
    }
}
