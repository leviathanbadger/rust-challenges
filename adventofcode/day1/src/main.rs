use std::{fs};
use anyhow::*;

fn get_calibration_value(s: &str) -> Result<u32> {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    for chr in s.chars() {
        let dig = chr.to_digit(10);
        if dig.is_some() {
            if first.is_none() {
                first = dig
            }
            last = dig
        }
    }

    if first.is_none() && last.is_none() {
        Err(anyhow!("No first or last character"))
    }
    else {
        Ok(first.unwrap() * 10 + last.unwrap())
    }
}

fn day1part1() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");

    let sum = input
        .split("\n")
        .map(get_calibration_value)
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .sum::<u32>();

    println!("Day 1 part 1 answer: {}", sum);
}

fn main() {
    day1part1();
}
