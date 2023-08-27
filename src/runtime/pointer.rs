use std::fmt;
use std::fmt::Display;
use std::ptr;

/// Wrapper around unsafe pointers. Allows to dereference outside unsafe blocks.
#[derive(Debug)]
pub struct Ptr<T> {
    pub value: *mut T,
}

impl <T> Ptr<T> {
    pub fn null() -> Ptr<T> {
        Ptr{value: ptr::null_mut()}
    }

    pub fn is_null(&self) -> bool {
        self.value.is_null()
    }

    pub fn as_ref(&self) -> &T {
        unsafe{ &*self.value }
    }

    pub fn as_mut_ref(&mut self) -> &mut T {
        unsafe{ &mut *self.value }
    }
}

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

impl <T: Display> fmt::Display for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
