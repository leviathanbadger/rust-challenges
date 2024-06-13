use core::result::Result::Ok;
use std::{fs, num::ParseIntError};
use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct AlmanacMapRange {
    dest_range_start: u64,
    src_range_start: u64,
    range_length: u64
}

#[derive(Debug)]
struct AlmanacMap {
    src: String,
    dest: String,
    ranges: Vec<AlmanacMapRange>
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>
}

lazy_static! {
    static ref MAP_RANGE_REGEX: Regex = Regex::new(r"^([0-9]+) +([0-9]+) +([0-9]+)$").unwrap();
    static ref MAP_NAMES_REGEX: Regex = Regex::new(r"^([a-z]+)-to-([a-z]+) map:$").unwrap();
    static ref SEEDS_LIST_REGEX: Regex = Regex::new(r"^seeds: (.*)$").unwrap();
}

fn parse_map_range(line: &str) -> Result<AlmanacMapRange> {
    let capture_opt = MAP_RANGE_REGEX.captures(line);
    match capture_opt {
        None => Err(anyhow!("Could not parse map range from line")),
        Some(capture) => {
            let (_, [dest_range_start_str, src_range_start_str, range_length_str]) = capture.extract();
            let dest_range_start = dest_range_start_str.parse::<u64>()?;
            let src_range_start = src_range_start_str.parse::<u64>()?;
            let range_length = range_length_str.parse::<u64>()?;
            Ok(AlmanacMapRange {
                dest_range_start,
                src_range_start,
                range_length
            })
        }
    }
}

fn parse_map_names(line: &str) -> Result<AlmanacMap> {
    let capture_opt = MAP_NAMES_REGEX.captures(line);
    match capture_opt {
        None => Err(anyhow!("Could not parse map names from line")),
        Some(capture) => {
            let (_, [src, dest]) = capture.extract();
            Ok(AlmanacMap {
                src: src.to_owned(),
                dest: dest.to_owned(),
                ranges: vec![]
            })
        }
    }
}

fn parse_seeds(line: &str) -> Result<Vec<u64>> {
    let capture_opt = SEEDS_LIST_REGEX.captures(line);
    match capture_opt {
        None => Err(anyhow!("Could not parse seeds from line")),
        Some(capture) => {
            let (_, [seed_nums_str]) = capture.extract();
            let seed_nums = seed_nums_str.split(' ')
                .filter(|seed_str| !seed_str.is_empty())
                .map(|seed_str| seed_str.parse::<u64>())
                .collect::<Result<Vec<u64>, ParseIntError>>()?;
            Ok(seed_nums)
        }
    }
}

fn parse_almanac(input: &String) -> Result<Almanac> {
    let mut seeds: Option<Vec<u64>> = None;
    let mut maps: Vec<AlmanacMap> = vec![];
    let mut current_map: Option<AlmanacMap> = None;

    for line in input.lines().filter(|line| !line.is_empty()) {
        if let Ok(new_map_range) = parse_map_range(line) {
            if let Some(ref mut current_map_unwrapped) = current_map {
                current_map_unwrapped.ranges.push(new_map_range);
            }
            else {
                return Err(anyhow!("Parsed map range before a map was created."));
            }
        }
        else if let Ok(new_map) = parse_map_names(line) {
            if let Some(current_map_unwrapped) = current_map {
                maps.push(current_map_unwrapped);
            }
            current_map = Some(new_map);
        }
        else if let Ok(new_seeds) = parse_seeds(line) {
            if seeds.is_some() {
                return Err(anyhow!("Seeds defined multiple times."));
            }
            else {
                seeds = Some(new_seeds);
            }
        }
        else {
            println!("Failed to parse line! {:?}", line);
        }
    }

    if let Some(current_map_unwrapped) = current_map {
        maps.push(current_map_unwrapped);
    }

    if seeds.is_none() {
        Err(anyhow!("Seeds are never defined."))
    }
    else {
        Ok(Almanac {
            seeds: seeds.unwrap(),
            maps
        })
    }
}

fn compute_mapping_route(almanac: &Almanac, src_name: &String, dest_name: &String) -> Result<Vec<usize>> {
    let mut route = vec![];

    let mut current_name = src_name;
    while current_name != dest_name {
        let next = almanac.maps
            .iter()
            .enumerate()
            .filter(|(_, map)| map.src == *current_name)
            .next();
        if let Some((idx, map)) = next {
            route.push(idx);
            current_name = &map.dest;
        }
        else {
            return Err(anyhow!("Could not find map with source name {:?}", current_name))
        }
    }

    Ok(route)
}

fn compute_single_mapping(map: &AlmanacMap, input: u64) -> u64 {
    for range in map.ranges.iter() {
        if input >= range.src_range_start && input < range.src_range_start + range.range_length {
            return input - range.src_range_start + range.dest_range_start;
        }
    }

    input
}

fn compute_mapping(almanac: &Almanac, route: &Vec<usize>, input: u64) -> u64 {
    route.iter()
        .fold(input, |input, idx| compute_single_mapping(&almanac.maps[*idx], input))
}

fn day5part1(almanac: &Almanac) -> Result<u64> {
    let route = compute_mapping_route(almanac, &"seed".to_owned(), &"location".to_owned())?;

    let min_location_opt = almanac.seeds
        .iter()
        .map(|seed| compute_mapping(almanac, &route, *seed))
        .min();

    if let Some(min_location) = min_location_opt {
        Ok(min_location)
    }
    else {
        Err(anyhow!("There was no min value. Are there no seeds?"))
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    let almanac = parse_almanac(&input)?;

    println!("Day 5 part 1 answer: {}", day5part1(&almanac)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5part1_returns_correct_value() -> Result<()> {
        const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let almanac = parse_almanac(&INPUT.to_owned())?;

        assert_eq!(day5part1(&almanac)?, 35);

        Ok(())
    }
}
