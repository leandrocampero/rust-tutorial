#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::{array, io};

fn say_hello() {
    println!("Hello world!")
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y)
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;

    for &val in list.iter() {
        sum += &val;
    }

    return sum;
}

fn main() {
    say_hello();
    get_sum(4, 6);
    println!("{}", get_sum_3(5, 7));

    let (val1, val2) = get(3);
    println!("Nums: {} {}", val1, val2);

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list))
}
