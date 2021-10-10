#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> (usize, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let num: usize = match args.get(1) {
        Some(num) => num.parse::<usize>().unwrap(),
        None => panic!("Usage: ./app number"),
    };
    let file_name = format!("./input/input_{:02}.txt", num);
    let lines: Vec<String> = BufReader::new(File::open(file_name).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect();
    (num, lines)
}

mod aoc_01;
mod aoc_02;
mod aoc_03;
mod aoc_04;
mod aoc_05;
mod aoc_06;
mod aoc_07;
mod aoc_08;
mod aoc_09;
mod aoc_10;
mod aoc_11;
mod aoc_12;
mod aoc_13;
mod aoc_14;
mod aoc_15;
mod aoc_16;

fn main() {
    let fns: Vec<fn(&Vec<String>) -> (Option<i64>, Option<i64>)> = vec![
        aoc_01::solve,
        aoc_02::solve,
        aoc_03::solve,
        aoc_04::solve,
        aoc_05::solve,
        aoc_06::solve,
        aoc_07::solve,
        aoc_08::solve,
        aoc_09::solve,
        aoc_10::solve,
        aoc_11::solve,
        aoc_12::solve,
        aoc_13::solve,
        aoc_14::solve,
        aoc_15::solve,
        aoc_16::solve,
    ];
    let (num, lines) = read_input();
    if num > fns.len() {
        panic!("Unknown puzzle num: {}", num);
    }
    let (first, second) = fns[num - 1](&lines);
    println!("Day {}, first: {:?}, second: {:?}", num, first, second);
}
