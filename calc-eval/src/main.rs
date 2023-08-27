#![feature(lazy_cell)]

mod calculator;

use anyhow::Result;
use std::io::Write;
use core::cell::LazyCell;
use calculator::Calculator;

fn read_user_input() -> Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(String::from(buffer.trim()))
}

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();

    let calc = LazyCell::new(|| {
        Calculator::new()
    });

    println!("Enter expressions to evaluate, or \".exit\" to exit.");

    loop {
        print!(" > ");
        stdout.flush()?;
        let line = read_user_input()?;

        if line == ".exit" {
            break;
        }

        let result = calc.eval(line);
        match result {
            Ok(n) => println!("{}", n),
            Err(_) => println!("There was an error evaluating your input.")
        }
    }

    Ok(())
}
