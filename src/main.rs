#![allow(dead_code, unused_imports, unused_variables)]
mod const_generics;
mod duration;
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
    duration_trait();
    vec_wrapper();
    square::exec_square();
    const_generics::ordered_array();
}

fn duration_trait() {
    let days = duration::Days::new(10); // foreign type

    let duration1: duration::Duration = duration::Duration::from(days); // explicit conversion
    let duration1a: duration::Duration = days.into(); // implicit conversion

    let secs = duration::Seconds::new(86400); // foreign type

    let duration2: duration::Duration = duration::Duration::from(secs); // explicit conversion
    let duration2a: duration::Duration = secs.into(); // implicit conversion

    // println!("{:?}", duration1);
    // println!("{:?}", duration1a);
    // println!("{:?}", duration2);
    // println!("{:?}", duration2a);

    // Flexibility
    duration::print_duration(duration::Days::new(10).into());
    duration::print_duration(duration::Seconds::new(864000).into());
    duration::print_duration(std::time::Duration::from_secs(864000).into());
}

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
