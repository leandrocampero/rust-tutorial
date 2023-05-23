#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::ops::Add;
use std::{array, io};

fn main() {
    const PI: f32 = 3.14159265;
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 main st."),
        balance: 234.50,
    };

    bob.address = String::from("505 2nd St");

    trait Shape {
        // Sounds like an Interface in Object Oriented Languages
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    }

    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 20.0);
    let circ: Circle = Shape::new(8.0, 2.0);

    println!(
        "Rec of {} by {} whose area is {}",
        rec.length,
        rec.width,
        rec.area()
    );
    println!(
        "Circle of {} by {} whose area is {}",
        circ.length,
        circ.width,
        circ.area()
    )
}
