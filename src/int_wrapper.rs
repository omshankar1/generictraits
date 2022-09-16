// ********************************************************
// Wrapper type for T
// ********************************************************
use std::fmt;
use std::ops::Deref;

pub struct Wrapper<T>(T);
impl<T> Wrapper<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "T {:?}", self.0)
    }
}
impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
