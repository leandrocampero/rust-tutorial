#![allow(unused)]

use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{array, io};

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    pub struct Bank {
        balance: f32,
    }

    /*
        fn withdraw(the_bank: &mut Bank, amount: f32) {
            the_bank.balance -= amount;
        }

        let mut bank = Bank { balance: 100.0 };
        withdraw(&mut bank, 5.00);
        println!("Balance: {}", bank.balance);

        fn customer(the_bank: &mut Bank) {
            withdraw(the_bank, 5.00)
        }

        // This is wrong because the closure can't borrow a value (in this case bank)
        // if it outlives the current function (in this case, main)
        thread::spawn(|| customer(&mut bank)).join().unwrap();
    */

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amount: f32) {
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00 {
            println!(
                "Current balance: {}; Withdrawal a smaller amount",
                bank_ref.balance
            )
        } else {
            bank_ref.balance -= amount;
            println!(
                "Customer withdrew {}; Current balance: {}",
                amount, bank_ref.balance
            )
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.0 }));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        return thread::spawn(|| customer(bank_ref));
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);
}
