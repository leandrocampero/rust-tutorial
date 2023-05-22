#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::ops::Add;
use std::{array, io};

fn print_string(x: String) {
    println!("A string: {}", x)
}

fn print_return_string(x: String) -> String {
    println!("Another string: {}", x);
    return x;
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name)
}

fn main() {
    let str1 = String::from("World");

    // // let str2 = str1;
    // // By doing this, the value of str1 is passed to str2. And now, str1 no longer exists
    // //println!("Hello {}!", str1) // Error

    let str2 = str1.clone();
    println!("Hello {}!", str1); // Works fine

    let str3 = print_return_string(str1);
    println!("str3 = {}", str3);

    let mut str4 = String::from("Derek");
    change_string(&mut str4)
}
