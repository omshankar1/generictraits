// Generic trait:
// Multiple implementation of the same trait for the same type
// with different Generic parameters

// Self: Type U(can be arbitrary)
// Type T is the Generic parameter (can be arbitrary)
// pub trait From<T>: Sized {
//     fn from(_: T) -> Self;
// }

// ********************************************************
// foreign types
// FROM (foreign type)
// ********************************************************
#[derive(Clone, Copy)]
pub struct Days(u16);
impl Days {
    pub fn new(day: u16) -> Self {
        Self(day)
    }
}

#[derive(Clone, Copy)]
pub struct Seconds(u64);
impl Seconds {
    pub fn new(sec: u64) -> Self {
        Self(sec)
    }
}
// std::time::duration

// ********************************************************
// TO Our type
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
