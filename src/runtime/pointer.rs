use std::fmt;
use std::fmt::Display;

use super::value::Value;

/// Newly created Ptr<Value> instances point to a static unit value rather than being null
static UNIT: Value = Value::Unit;

/// Wrapper around unsafe pointers. Allows to dereference outside unsafe blocks.
#[derive(Debug)]
pub struct Ptr<T> {
    value: *const T,
}

impl Ptr<Value> {
    pub fn unit() -> Ptr<Value> {
        Ptr { value: &UNIT as *const Value as *mut Value }
    }
}

impl <T> Ptr<T> {
    pub fn as_ref(&self) -> &T {
        unsafe{ &*self.value }
    }

    pub fn as_mut_ref(&mut self) -> &mut T {
        unsafe{ &mut *(self.value as *mut T) }
    }
}

unsafe impl<T> Sync for Ptr<T> {}

impl<T> Copy for Ptr<T> {}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

impl <T> From<T> for Ptr<T> {
    fn from(value: T) -> Self {
        Self::from(Box::new(value))
    }
}

impl <T> From<Box<T>> for Ptr<T> {
    fn from(value: Box<T>) -> Self {
        Self { value: Box::into_raw(value) }
    }
}

impl <T> From<&T> for Ptr<T> {
    fn from(value: &T) -> Self {
        Self { value: (value as *const T) as *mut T }
    }
}

impl <T> PartialEq for Ptr<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.value, other.value)
    }
}

impl <T: Display> fmt::Display for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
