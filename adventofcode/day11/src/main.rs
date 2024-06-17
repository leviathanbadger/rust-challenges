use core::result::Result::Ok;
use std::{collections::HashSet, fs};
use anyhow::*;
use itertools::Itertools;

fn parse_galaxy_map(input: &String) -> Vec<(usize, usize)> {
    let galaxies_before_expansion = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, chr)| **chr == b'#')
            .map(move |(x, _)| (x, y)))
        .collect::<Vec<(usize, usize)>>();

    let gp_horiz = HashSet::<usize>::from_iter(galaxies_before_expansion
        .iter()
        .map(|(x, _)| *x)
    );

    let gp_vert = HashSet::<usize>::from_iter(galaxies_before_expansion
        .iter()
        .map(|(_, y)| *y)
    );

    let mut expand_horiz = (0..(*gp_horiz.iter().max().unwrap_or(&0)))
        .filter(|x| !gp_horiz.contains(x))
        .collect::<Vec<usize>>();
    expand_horiz.sort_unstable();
    let mut expand_vert = (0..(*gp_vert.iter().max().unwrap_or(&0)))
        .filter(|y| !gp_vert.contains(y))
        .collect::<Vec<usize>>();
    expand_vert.sort_unstable();

    galaxies_before_expansion
        .iter()
        .map(|(x, y)| (*x + expand_horiz.iter().filter(|tx| **tx < *x).count(), *y + expand_vert.iter().filter(|ty| **ty < *y).count()))
        .collect::<Vec<(usize, usize)>>()
}

fn day11part1(map: &Vec<(usize, usize)>) -> usize {
    let pairs = map
        .iter()
        .tuple_combinations()
        .collect::<Vec<(&(usize, usize), &(usize, usize))>>();

    pairs
        .iter()
        .map(|(a, b)| (i32::abs((b.0 as i32) - (a.0 as i32)) + i32::abs((b.1 as i32) - (a.1 as i32))) as usize)
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    let map = parse_galaxy_map(&input);

    println!("Day 11 part 1 answer: {}", day11part1(&map));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11part1_returns_correct_value() -> Result<()> {
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
        let map = parse_galaxy_map(&INPUT.to_owned());

        assert_eq!(day11part1(&map), 374);

        Ok(())
    }
}
