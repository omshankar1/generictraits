pub trait Square: std::fmt::Debug {
    type RET;
    fn sqr(&self) -> Self::RET;
}

// Implementation of Square for u32
impl Square for u32 {
    type RET = u32;
    fn sqr(&self) -> Self::RET {
        self * self
    }
}
// Implementation of Square for f32
impl Square for f32 {
    type RET = f32;
    fn sqr(&self) -> Self::RET {
        self * self
    }
}

pub fn exec_square() {
    println!("Square of 10.1: {:?}", (10.1).sqr());
    println!("Square of 10: {:?}", 10.sqr());
    println!();
}
