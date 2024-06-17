use core::result::Result::Ok;
use std::{collections::HashSet, fs};
use anyhow::*;
use itertools::Itertools;

fn parse_galaxy_map(input: &String, expand_amt: u64) -> Vec<(u64, u64)> {
    let galaxies_before_expansion = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, chr)| **chr == b'#')
            .map(move |(x, _)| (x as u64, y as u64)))
        .collect::<Vec<(u64, u64)>>();

    let gp_horiz = HashSet::<u64>::from_iter(galaxies_before_expansion
        .iter()
        .map(|(x, _)| *x)
    );

    let gp_vert = HashSet::<u64>::from_iter(galaxies_before_expansion
        .iter()
        .map(|(_, y)| *y)
    );

    let mut expand_horiz = (0..(*gp_horiz.iter().max().unwrap_or(&0)))
        .filter(|x| !gp_horiz.contains(x))
        .collect::<Vec<u64>>();
    expand_horiz.sort_unstable();
    let mut expand_vert = (0..(*gp_vert.iter().max().unwrap_or(&0)))
        .filter(|y| !gp_vert.contains(y))
        .collect::<Vec<u64>>();
    expand_vert.sort_unstable();

    galaxies_before_expansion
        .iter()
        .map(|(x, y)| (*x + (expand_horiz.iter().filter(|tx| **tx < *x).count() as u64) * expand_amt, *y + (expand_vert.iter().filter(|ty| **ty < *y).count() as u64) * expand_amt))
        .collect::<Vec<(u64, u64)>>()
}

fn absdiff(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    }
    else {
        b - a
    }
}

fn day11(input: &String, expand_amt: u64) -> u64 {
    let map = parse_galaxy_map(&input, expand_amt);

    let pairs = map
        .iter()
        .tuple_combinations()
        .collect::<Vec<(&(u64, u64), &(u64, u64))>>();

    pairs
        .iter()
        .map(|(a, b)| absdiff(a.0, b.0) + absdiff(a.1, b.1))
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");

    println!("Day 11 part 1 answer: {}", day11(&input, 1));
    println!("Day 11 part 2 answer: {}", day11(&input, 1000000 - 1));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_returns_correct_value() {
        const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(day11(&INPUT.to_owned(), 1), 374);
    }

    #[test]
    fn day11_returns_correct_value_2() {
        const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(day11(&INPUT.to_owned(), 9), 1030);
    }

    #[test]
    fn day11_returns_correct_value_3() {
        const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(day11(&INPUT.to_owned(), 99), 8410);
    }
}
