#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::{array, io};

fn main() {
    let mut st1 = String::new();

    st1.push('N');
    st1.push_str(" word");

    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("N", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char)
    }

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];

    println!("String length: {}", st6.len());
    st5.clear();
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7; //st6 is no longer available, it's in st88 now. But st7 does exists beacuse I called it by reference

    for char in st8.bytes() {
        println!("{}", char)
    }
}
