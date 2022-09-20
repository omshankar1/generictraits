// pub trait From<T>: Sized {
//     fn from(_: T) -> Self;
// }

// ********************************************************
// foreign types: Days, Seconds
// internal type: Duration
// Problem: Convert from Days/Seconds to Duration
// Solution: using trait FROM (foreign type) to (local type)
// ********************************************************

// ********   Days         ********************************
#[derive(Debug, Clone, Copy)]
pub struct Days(u16);
impl Days {
    pub fn new(day: u16) -> Self {
        Self(day)
    }
}

// ********   Seconds         ****************************
#[derive(Debug, Clone, Copy)]
pub struct Seconds(u64);
impl Seconds {
    pub fn new(sec: u64) -> Self {
        Self(sec)
    }
}

// ********************************************************
// TO (Our type: Duration)
// ********************************************************
#[derive(Debug)]
pub struct Duration {
    days: u64,
}
// ********************************************************

impl From<Days> for Duration {
    fn from(days: Days) -> Self {
        Self {
            days: days.0 as u64,
        }
    }
}
impl From<Seconds> for Duration {
    fn from(secs: Seconds) -> Self {
        Self {
            days: secs.0 / (24 * 3600),
        }
    }
}
impl From<std::time::Duration> for Duration {
    fn from(duration: std::time::Duration) -> Self {
        Self {
            days: duration.as_secs() / (24 * 3600),
        }
    }
}

pub fn print_duration(duration: Duration) {
    println!("Number of days: {}", duration.days);
}

// Generic trait:
// Definition: Multiple implementations of the same trait for the same type
// with different Generic parameters

// Implementation: on 'Self' (Type U)
// Trait: 'From' (Defined in std lib)
// Generic Parameter: Type 'T'
