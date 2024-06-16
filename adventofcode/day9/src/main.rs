use core::result::Result::Ok;
use std::{fs, num::ParseIntError};
use anyhow::*;
use itertools::Itertools;

fn parse_histories(input: &String) -> Result<Vec<Vec<i64>>> {
    let output = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(' ')
            .filter(|num_str| !num_str.is_empty())
            .map(|num_str| num_str.parse::<i64>())
            .collect::<Result<Vec<i64>, ParseIntError>>())
        .collect::<Result<Vec<Vec<i64>>, ParseIntError>>()?;

    Ok(output)
}

fn extrapolate_next(history: &Vec<i64>) -> i64 {
    let mut differences = vec![history.clone()];

    loop {
        let previous = &differences[differences.len() - 1];
        if previous.iter().all(|n| *n == 0) {
            break;
        }
        let next = previous
            .iter()
            .tuple_windows()
            .map(|(a, b)| *b - *a)
            .collect::<Vec<i64>>();
        differences.push(next);
    }

    let last_idx = differences.len() - 1;
    differences[last_idx].push(0);

    for q in (0..last_idx).rev() {
        let after = &differences[q + 1];
        let diff = after[after.len() - 1];
        let current = &mut differences[q];
        current.push(current[current.len() - 1] + diff);
    }

    let first = &differences[0];
    first[first.len() - 1]
}

fn day9part1(histories: &Vec<Vec<i64>>) -> i64 {
    histories
        .iter()
        .map(|history| extrapolate_next(history))
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    let histories = parse_histories(&input)?;

    println!("Day 9 part 1 answer: {}", day9part1(&histories));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9part1_returns_correct_value() -> Result<()> {
        const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let histories = parse_histories(&INPUT.to_owned())?;

        assert_eq!(day9part1(&histories), 114);

        Ok(())
    }
}
