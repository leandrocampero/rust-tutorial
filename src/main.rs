#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::ops::Add;
use std::{array, io};

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    /**
     * Crates: Modules that produce a library or executable
     * Modules: Organize and handle privacy
     * Packages: Build, test and share crates
     * Paths: A way of naming an item such as a struct, function
     */
    order_food();
}
