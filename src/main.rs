#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::{array, io};

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    let mut array = [1, 2, 3, 4];

    for val in array.iter() {
        // This way the loop is "borrowing the values from the array. Can't manipulate it"
        println!("{}", val)
    }

    let mut iter1 = array.iter();

    println!("1st: {:?}", iter1.next());

    let can_vote = |age: i32| -> bool { age >= 18 };
    // It's la Javascript's Arrow Functions or Java's Lambda Functions

    println!("Can vote?: {}", can_vote(9));

    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();

    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();

    println!("samp1 = {}", samp1);

    samp1 = 10;
    println!("samp1 = {}", samp1);
}
