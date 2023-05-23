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
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }

    let sum = |a: i32, b: i32| a + b;
    let prod = |a: i32, b: i32| a * b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}
