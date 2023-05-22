#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::{array, io};

fn main() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st: {}", arr_1[0]);
    println!("length: {}", arr_1.len());

    let mut loop_index = 0;

    loop {
        if arr_1[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        if arr_1[loop_index] == 9 {
            break;
        }
        println!("Val: {}", arr_1[loop_index]);
        loop_index += 1;
    }

    loop_index = 0;
    while loop_index < arr_1.len() {
        println!("Array with 'while': {}", arr_1[loop_index]);
        loop_index += 1;
    }

    loop_index = 0;
    for val in arr_1.iter() {
        println!("Array with 'for': {}", val);
    }
}
