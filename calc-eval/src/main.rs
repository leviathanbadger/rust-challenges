use anyhow::*;
use std::io::Write;

fn read_user_input() -> Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(String::from(buffer.trim()))
}

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();

    println!("Enter expressions to evaluate, or \".exit\" to exit.");

    loop {
        print!(" > ");
        stdout.flush()?;
        let line = read_user_input()?;

        if line == ".exit" {
            break;
        }

        println!("{}", line);
    }

    Ok(())
}
