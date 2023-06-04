#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::thread;
use std::time::Duration;
use std::{array, io};

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();
}
