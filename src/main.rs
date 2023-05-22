#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufReader, ErrorKind, Write};

fn main() {
    let age = 8;

    if (age >= 1) && (age <= 18) {
        println!("Important birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important birthday");
    } else if age >= 65 {
        println!("Important birthday");
    } else {
        println!("Not an important birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote?: {}", can_vote);

    let age2 = 8;
    match age2 {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("Important birthday"),
        65.. => println!("Important birthday"),
        _ => println!("Not an important birthday"),
    };

    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };
}
