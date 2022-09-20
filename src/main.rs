#![allow(dead_code, unused_imports, unused_variables)]
mod const_generics;
mod distance;
mod duration;
mod int_wrapper;
mod point;
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

    // Trait defined on primitive eg: 2.square()
    square::exec_square();

    // Trait : From and Add
    add_distance();

    // Deref trait
    int_wrapper();

    // add_points();
    // vec_wrapper();
    // const_generics::ordered_array();
}

fn duration_trait() {
    // Generic Traits - 'From'
    // Foreign Type: Create an instance Days type
    let days = duration::Days::new(10); // foreign type

    // Explicit conversion from foreign(Days) to Internal(Duration)
    let duration1: duration::Duration = duration::Duration::from(days);
    // More succinct implicit conversion
    let duration1a: duration::Duration = days.into();

    // Foreign Type: Create an instance Seconds type
    let days = duration::Days::new(10); // foreign type
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
    println!();
}
fn add_distance() {
    let meters = distance::Meter(100.0);
    let mm = distance::Millimeter(100000.0);
    let result: distance::Meter = meters + mm;
    println!("Meter-> meters+mm: {:?}", result);
    let result: distance::Millimeter = (meters + mm).into();
    println!("Millimeter-> meters+mm: {:?}", result);
    let result: distance::Millimeter = mm + meters;
    println!("Millimeter-> mm+meters: {:?}", result);
}

// Int wrapper - Deref trait
fn int_wrapper() {
    let int_wrapper = int_wrapper::Wrapper::new(5);
    let val = &*int_wrapper;
    println!("Int Wrapper: {:?}", val);
}

fn add_points() {
    let p1 = point::Point { x: 1, y: 1 };
    let p2 = point::Point { x: 2, y: 2 };
    let p3 = p1 + p2;
    println!("Adding p1+p2: {:?}", p3);
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
