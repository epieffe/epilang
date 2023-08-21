use std::fmt;
use std::ptr;

use crate::intermediate::constant::Constant;
use crate::intermediate::constant::Type;
use crate::intermediate::exp::Exp;

#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Function(Function),
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Unit => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Function(_) => true,
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Value::Unit => Type::Unit,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Function(_) => Type::Function,
        }
    }
}

impl From<&Constant> for Value {
    fn from(value: &Constant) -> Self {
        match value {
            Constant::Unit => Value::Unit,
            Constant::Int(i) => Value::Int(*i),
            Constant::Float(f) => Value::Float(*f),
            Constant::String(s) => Value::String(s.clone()),
            Constant::Bool(b) => Value::Bool(*b),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "unit"),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(float) => write!(f, "{}", float),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Function(func) => write!(f, "[Function {:p}]", func),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub num_args: usize,
    pub external_values: Vec<Value>,
    pub body: Box<Exp>
}

#[derive(Copy, Clone, Debug)]
pub struct Pointer {
    pub value: *mut Value,
}

impl Pointer {
    pub fn unit() -> Pointer {
        Pointer{value: ptr::null_mut()}
    }

    pub fn is_unit(&self) -> bool {
        self.value.is_null()
    }

    pub fn as_ref(&self) -> &Value {
        unsafe{ &*self.value }
    }
}

impl From<Box<Value>> for Pointer {
    fn from(value: Box<Value>) -> Self {
        Pointer { value: Box::into_raw(value) }
    }
}

impl fmt::Display for Pointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug)]
pub enum V {
    Val(Value),
    Ptr(Pointer),
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
