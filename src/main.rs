#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufReader, ErrorKind, Write};

fn main() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.000000000000000222);

    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.000000000000000222);

    /*
    f32: 1.1111112
    f64: 1.1111111111111112
     */

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("{} + {} = {}", num_3, num_4, num_3 + num_4,);
    println!("{} - {} = {}", num_3, num_4, num_3 - num_4,);
    println!("{} * {} = {}", num_3, num_4, num_3 * num_4,);
    println!("{} / {} = {}", num_3, num_4, num_3 / num_4,);
    println!("{} % {} = {}", num_3, num_4, num_3 % num_4,);

    /*
    5 + 4 = 9
    5 - 4 = 1
    5 * 4 = 20
    5 / 4 = 1
    5 % 4 = 1
     */

    let random_num = rand::thread_rng().gen_range(1..101); // Doesn't require changing the seed, like C
    println!("Random: {}", random_num)
}
