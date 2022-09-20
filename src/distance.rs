use std::ops::Add;

// ****** Struct Meter ***************************
#[derive(Debug, Clone, Copy)]
pub struct Meter(pub f32);

impl Meter {
    pub fn new(val: f32) -> Self {
        Self(val)
    }
}
// Impl of Trait: Add - Millimeter + Millimeter
impl Add<Self> for Millimeter {
    type Output = Self;
    fn add(self, other: Millimeter) -> Self {
        Self::new(self.0 + other.0)
    }
}
// Impl of Trait: Add - Millimeter + Meter
impl Add<Meter> for Millimeter {
    type Output = Self;
    fn add(self, other: Meter) -> Self {
        Self::new(self.0 + (other.0 * 1000.0))
    }
}
// Impl of Trait: From
impl From<Meter> for Millimeter {
    fn from(m: Meter) -> Self {
        Self::new(m.0 * 100.0)
    }
}

// ****** Struct Millimeter ***************************
#[derive(Debug, Clone, Copy)]
pub struct Millimeter(pub f32);
impl Millimeter {
    pub fn new(val: f32) -> Self {
        Self(val)
    }
}
// Impl of Trait: Add Meter + Meter
impl Add<Self> for Meter {
    type Output = Self;
    fn add(self, other: Meter) -> Self {
        Self::new(self.0 + other.0)
    }
}
// Impl of Trait: Add Meter + Millimeter
impl Add<Millimeter> for Meter {
    type Output = Self;
    fn add(self, other: Millimeter) -> Self {
        Self::new(self.0 + (other.0 / 1000.0))
    }
}

// Impl of Trait: From
impl From<Millimeter> for Meter {
    fn from(mm: Millimeter) -> Self {
        Self::new(mm.0 / 100.0)
    }
}
