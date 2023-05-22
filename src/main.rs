#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::{array, io};

fn main() {
    // Casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;

    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // Enums
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    };

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Days = Days::Monday;
    match today {
        Days::Monday => println!("Everyone hates monday"),
        Days::Tuesday => println!("Donut day"),
        Days::Wednesday => println!("Hump day"),
        Days::Thursday => println!("Pay day"),
        Days::Friday => println!("Almost weekend"),
        Days::Saturday => println!("Weekend!"),
        Days::Sunday => println!("Weekend!"),
    }

    println!("Is today the weekend?: {}", today.is_weekend());
}
