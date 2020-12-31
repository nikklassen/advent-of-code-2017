#![allow(dead_code)]

extern crate regex;

pub mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    println!("{}", day05::part2());
}
