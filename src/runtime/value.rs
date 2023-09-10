use std::fmt;
use std::collections::HashMap;

use crate::intermediate::constant::{Constant, Type};
use crate::intermediate::exp::BuiltInFunction;

use super::pointer::Ptr;
use super::function::{Function, Method, BuiltInMethod};

#[derive(Debug)]
pub enum Value {
    Unit,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    List(Vec<Ptr<Value>>),
    Function(Function),
    BuiltInFunction(BuiltInFunction),
    Class(Ptr<Class>),
    Object(Object),
    Method(Method),
    BuiltInMethod(BuiltInMethod),
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
            _ => true,
        }
    }

    pub fn get_field(&self, name: &str) -> Option<Ptr<Value>> {
        match self {
            Value::Object(o) => o.get_field(name),
            _ => None,
        }
    }

    pub fn get_method(&self, name: &str) -> Option<Method> {
        match self {
            Value::Object(o) => {
                o.get_method(name).map(|function| {
                    Method { self_value: Ptr::from(self), function }
                })
            },
            _ => None
        }
    }

    pub fn get_builtin_methd(&self, name: &str) -> Option<BuiltInMethod> {
        match self {
            Value::List(_) => {
                match name {
                    "len" => Some(BuiltInFunction::ListLength),
                    "push" => Some(BuiltInFunction::ListPush),
                    "pop" => Some(BuiltInFunction::ListPop),
                    "remove" => Some(BuiltInFunction::ListRemove),
                    _ => None
                }.map(|function| {
                    BuiltInMethod { self_value: Ptr::from(self), function }
                })
            },
            _ => None
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
            Value::BuiltInFunction(_) => Type::Function,
            Value::Class(_) => Type::Class,
            Value::Object(_) => Type::Object,
            Value::Method(_) => Type::Method,
            Value::BuiltInMethod(_) => Type::Method,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::String(s) => format!("\"{}\"", s.clone()),
            v => format!("{}", v)
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
            Value::Function(func) => write!(f, "[Function at {:p}]", func),
            Value::BuiltInFunction(func) => write!(f, "[Function at {:p}]", func),
            Value::Class(class) => write!(f, "[Class {} at {:p}]", class.as_ref().name, class.as_ref()),
            Value::Object(o) => write!(f, "[{} object at {:p}]", o.class.as_ref().name, o),
            Value::Method(m) => write!(f, "[Method at {:p}]", m),
            Value::BuiltInMethod(m) => write!(f, "[Method at {:p}]", m),
        }
    }
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
    pub constructor: Function,
    pub methods: HashMap<String, Ptr<Function>>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
}

#[derive(Debug)]
pub struct Object {
    pub class: Ptr<Class>,
    pub fields: HashMap<String, Ptr<Value>>,
}

impl Object {
    pub fn get_field(&self, name: &str) -> Option<Ptr<Value>> {
        self.fields.get(name).copied()
    }

    pub fn get_mut_field(&mut self, name: &str) -> Option<&mut Ptr<Value>> {
        self.fields.get_mut(name)
    }

    pub fn get_method(&self, name: &str) -> Option<Ptr<Function>> {
        self.class.as_ref().methods.get(name).copied()
    }
}

#[derive(Debug)]
pub enum V {
    Ptr(Ptr<Value>),
    Val(Value),
}

impl V {
    pub fn as_bool(&self) -> bool {
        match self {
            V::Ptr(ptr) => ptr.as_ref().as_bool(),
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

    pub fn into_ptr(self) -> Ptr<Value> {
        match self {
            V::Ptr(ptr) => ptr,
            V::Val(value) => Ptr::from(value)
        }
    }
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            V::Ptr(ptr) => write!(f, "{}", ptr.as_ref()),
            V::Val(value) => write!(f, "{}", value)
        }
    }
}
