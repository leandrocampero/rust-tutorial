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
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            // Also works without the curly braces
            panic!("Problem creating the file: {:?}", error)
        }
    };

    write!(output, "Just some text\nHello world!").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap())
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(file_created) => file_created,
                Err(e) => panic!("Can't create error: {:?}", e),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}
