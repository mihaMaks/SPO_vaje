#![allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // strings, mutable variables and stdin
    println!("What is your name?");
    let mut name = String::new();
    let greetings: &str = "Nice to meet you!";
    io::stdin()
        .read_line(&mut name)
        .expect("didnt recive input.");
    println!("Hello {} \n{}", name.trim_end(), greetings);
    // Const
    const ONE_MIL: u32 = 1_000_000;
    const PI: f64 = std::f64::consts::PI;
    println!("This is PI:{} and 10^6: {}", PI, ONE_MIL);
}
