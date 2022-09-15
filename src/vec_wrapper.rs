// ********************************************************
// Wrapper type for Vec<T>
// ********************************************************
use std::fmt;
use std::ops::Deref;

pub struct Wrapper<T>(Vec<T>);
impl<T> Wrapper<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self(v)
    }
}
impl<T> From<Wrapper<T>> for Vec<T> {
    fn from(w: Wrapper<T>) -> Vec<T> {
        w.0
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec {:?}", self.0)
    }
}
impl<T> Deref for Wrapper<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
