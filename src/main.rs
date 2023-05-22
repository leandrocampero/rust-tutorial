#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14159265;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    // shadowing: overload a variable as long as they have different data types

    age = age + 1;

    println!("I'm {} and I want ${}", age, ONE_MIL);
}
