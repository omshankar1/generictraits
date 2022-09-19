#![allow(dead_code, unused_imports, unused_variables)]
mod const_generics;
mod duration;
mod int_wrapper;
mod square;
mod vec_wrapper;
use const_generics::IsOrdered;
use square::Square;
use std::fmt;
use std::ops::Deref;
use std::{fmt::Debug, net::Ipv4Addr};

// ********************************************************

fn main() {
    // ********************************************************
    // From trait
    duration_trait();

    // Trait defined on primitive types without wrapping
    square::exec_square();

    // Deref trait
    int_wrapper();

    // vec_wrapper();
    // const_generics::ordered_array();
}

fn duration_trait() {
    // Generic Traits - 'From'
    // Create an instance of foreign type Days
    let days = duration::Days::new(10); // foreign type

    // Explicit conversion from foreign(Days) to Internal(Duration)
    let duration1: duration::Duration = duration::Duration::from(days);
    // More succinct implicit conversion
    let duration1a: duration::Duration = days.into();

    // Create an instance of foreign type Seconds
    let secs = duration::Seconds::new(86400); // foreign type

    // Explicit conversion from foreign(Seconds) to Internal(Duration)
    let duration2: duration::Duration = duration::Duration::from(secs);
    // More succinct implicit conversion
    let duration2a: duration::Duration = secs.into();

    // println!("{:?}", duration2a);

    // Flexibility
    duration::print_duration(duration::Days::new(10).into());
    duration::print_duration(duration::Seconds::new(864000).into());
    duration::print_duration(std::time::Duration::from_secs(864000).into());
}

// Int wrapper - Deref trait
fn int_wrapper() {
    let int_wrapper = int_wrapper::Wrapper::new(5);
    let val = &*int_wrapper;
    println!("Int Wrapper: {:?}", val);
}

// Vec Wrapper - From and Deref trait
fn vec_wrapper() {
    let vec_wrapper = vec_wrapper::Wrapper::new(vec![1, 3, 5]);
    let vec: Vec<u8> = Vec::<u8>::from(vec_wrapper);
    println!("{:?}", vec);

    let vec_wrapper2 = vec_wrapper::Wrapper::new(vec![1, 3, 5]);
    let vec2: Vec<u8> = vec_wrapper2.into();
    println!("{:?}", vec2);

    let vec_wrapper3 = vec_wrapper::Wrapper::new(vec![3, 9, 8]);
    let vec3 = &*vec_wrapper3;
    println!("{:?}", vec3);
}
