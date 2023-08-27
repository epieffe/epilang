use std::fmt;
use std::collections::HashMap;

use crate::intermediate::constant::Constant;
use crate::intermediate::constant::Type;
use crate::intermediate::exp::Exp;

use super::pointer::Ptr;

#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    List(Vec<Ptr<Value>>),
    Function(Function),
    Class(Ptr<Class>),
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Unit => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Function(_) => true,
            Value::Class(_) => true,
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Value::Unit => Type::Unit,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::List(_) => Type::List,
            Value::Function(_) => Type::Function,
            Value::Class(_) => Type::Class,
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
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::List(l) => {
                write!(f, "[")?;
                if l.len() > 0 {
                    for i in 0..l.len() - 1 {
                        write!(f, "{}, ", l[i])?;
                    }
                    write!(f, "{}", l[l.len() - 1])?;
                }
                write!(f, "]")
            },
            Value::Function(func) => write!(f, "[Function {:p}]", func),
            Value::Class(class) => write!(f, "[Class {}]", class.as_ref().name)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: HashMap<String, Ptr<Function>>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub num_args: usize,
    pub external_values: Vec<Ptr<Value>>,
    pub body: Box<Exp>
}

#[derive(Debug)]
pub enum V {
    Ptr(Ptr<Value>),
    Val(Value),
}

impl V {
    pub fn as_bool(&self) -> bool {
        match self {
            V::Ptr(ptr) => if ptr.is_null() {false} else {ptr.as_ref().as_bool()},
            V::Val(value) => value.as_bool()
        }
    }

    pub fn as_ref(&self) -> &Value {
        match self {
            V::Ptr(ptr) => ptr.as_ref(),
            V::Val(value) => value
        }
    }

    pub fn as_mut_ref(&mut self) -> &mut Value {
        match self {
            V::Ptr(ptr) => ptr.as_mut_ref(),
            V::Val(value) => value
        }
    }
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            V::Ptr(ptr) => if ptr.is_null() {write!(f, "unit")}  else {write!(f, "{}", ptr.as_ref())},
            V::Val(value) => write!(f, "{}", value)
        }
    }
}
