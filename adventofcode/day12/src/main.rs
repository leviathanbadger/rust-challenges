use core::result::Result::Ok;
use std::{fs, iter, num::ParseIntError};
use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
enum SpringCondition {
    Operational = b'.',
    Broken = b'#',
    Unknown = b'?'
}

impl TryFrom<u8> for SpringCondition {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b'.' => Ok(SpringCondition::Operational),
            b'#' => Ok(SpringCondition::Broken),
            b'?' => Ok(SpringCondition::Unknown),
            _ => Err(anyhow!("Could not convert u8 to SpringCondition"))
        }
    }
}

#[derive(Debug)]
struct SpringConditionRecord {
    condition: Vec<SpringCondition>,
    contiguous_damaged_springs: Vec<usize>
}

lazy_static! {
    static ref SPRING_CONDITIONS_REGEX: Regex = Regex::new(r"^([.#?]+) *([,0-9]+)$").unwrap();
}

fn parse_spring_condition_record(line: &str) -> Result<SpringConditionRecord> {
    let captures_opt = SPRING_CONDITIONS_REGEX.captures(line);
    match captures_opt {
        Some(captures) => {
            let (_, [condition_str, contiguous_str]) = captures.extract();

            let condition = condition_str.as_bytes()
                .iter()
                .map(|bit| SpringCondition::try_from(*bit))
                .collect::<Result<Vec<SpringCondition>>>()?;

            let contiguous_damaged_springs = contiguous_str.split(',')
                .filter(|num_str| !num_str.trim().is_empty())
                .map(|num_str| num_str.trim().parse::<usize>())
                .collect::<Result<Vec<usize>, ParseIntError>>()?;

            Ok(SpringConditionRecord {
                condition,
                contiguous_damaged_springs
            })
        },
        None => Err(anyhow!("Could not parse spring condition record from line: {:?}", line))
    }
}

fn count_possible_spring_arrangements_sub(record: &SpringConditionRecord, col_idx: usize, spring_idx: usize) -> u32 {
    if spring_idx >= record.contiguous_damaged_springs.len() {
        let has_remaining_definitely_broken = col_idx < record.condition.len() && (&record.condition[col_idx..]).iter().any(|condition| *condition == SpringCondition::Broken);
        return if has_remaining_definitely_broken { 0 } else { 1 };
    }

    let damage_len = record.contiguous_damaged_springs[spring_idx];
    let pad_right = (&record.contiguous_damaged_springs[(spring_idx + 1)..])
        .iter()
        .fold(0, |acc, next| acc + *next + 1);
    let max_possible_pos_i32 = record.condition.len() as i32 - pad_right as i32 - damage_len as i32;
    if col_idx as i32 > max_possible_pos_i32 {
        return 0;
    }
    let mut max_possible_pos = max_possible_pos_i32 as usize;

    let next_definitely_broken_opt = (&record.condition[col_idx..])
        .iter()
        .enumerate()
        .filter(|(_, condition)| **condition == SpringCondition::Broken)
        .map(|(idx, _)| idx + col_idx)
        .next();

    if let Some(next_definitely_broken) = next_definitely_broken_opt {
        max_possible_pos = usize::min(max_possible_pos, next_definitely_broken);
    }

    let mut total_options = 0;
    for q in col_idx..(max_possible_pos + 1) {
        let are_all_possibly_broken = record.condition[q..(q + damage_len)].iter().all(|condition| *condition != SpringCondition::Operational);
        if are_all_possibly_broken && (q == 0 || record.condition[q - 1] != SpringCondition::Broken) && (q + damage_len == record.condition.len() || record.condition[q + damage_len] != SpringCondition::Broken) {
            total_options += count_possible_spring_arrangements_sub(record, q + damage_len + 1, spring_idx + 1);
        }
    }

    total_options
}

fn count_possible_spring_arrangements(record: &SpringConditionRecord) -> u32 {
    count_possible_spring_arrangements_sub(record, 0, 0)
}

fn unfold_record(record: &mut SpringConditionRecord, times: usize) {
    let og_condition_length = record.condition.len();
    record.condition = record.condition
        .iter()
        .cloned()
        .chain(iter::once(SpringCondition::Unknown))
        .cycle()
        .take((og_condition_length + 1) * times - 1)
        .collect::<Vec<SpringCondition>>();

    let og_damages_length = record.contiguous_damaged_springs.len();
    record.contiguous_damaged_springs = record.contiguous_damaged_springs
        .iter()
        .cloned()
        .cycle()
        .take(og_damages_length * times)
        .collect::<Vec<usize>>();
}

fn day12part1(input: &String) -> Result<u32> {
    let records = input.lines()
        .map(|line| parse_spring_condition_record(line))
        .collect::<Result<Vec<SpringConditionRecord>>>()?;

    let result = records
        .iter()
        .map(|record| count_possible_spring_arrangements(record))
        .sum();

    Ok(result)
}

fn day12part2(input: &String) -> Result<u32> {
    let mut records = input.lines()
        .map(|line| parse_spring_condition_record(line))
        .collect::<Result<Vec<SpringConditionRecord>>>()?;

    for record in records.as_mut_slice() {
        unfold_record(record, 5);
    }

    let result = records
        .iter()
        .map(|record| count_possible_spring_arrangements(record))
        .sum();

    Ok(result)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");

    println!("Day 12 part 1 answer: {}", day12part1(&input)?);
    println!("Day 12 part 2 answer: {}", day12part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12part1_returns_correct_value() -> Result<()> {
        const INPUT: &str = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";

        assert_eq!(day12part1(&INPUT.to_owned())?, 6);

        Ok(())
    }

    #[test]
    fn day12part1_returns_correct_value_2() -> Result<()> {
        const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(day12part1(&INPUT.to_owned())?, 21);

        Ok(())
    }

    #[test]
    fn day12part2_returns_correct_value() -> Result<()> {
        const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(day12part2(&INPUT.to_owned())?, 525152);

        Ok(())
    }

    #[test]
    fn count_possible_spring_arrangements_returns_correct_value() -> Result<()> {
        let tests = vec![
            ("### 3", 1),
            (".?#?.?#?. 2,2", 4),
            (".?#?. 1", 1),
            (".?#?. 2", 2),
            (".?#?. 3", 1),
            (".???. 1", 3),
            (".???. 4", 0),

            ("#.#.### 1,1,3", 1),
            (".#...#....###. 1,1,3", 1),
            (".#.###.#.###### 1,3,1,6", 1),
            ("####.#...#... 4,1,1", 1),
            ("#....######..#####. 1,6,5", 1),
            (".###.##....# 3,2,1", 1),

            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),

            ("?#?##?#????.?..?? 9,1", 7),
            (".???.###. 3", 1),
            (".##.?#??.#.?# 2,1,1,1", 1),
            (".??.?#??.#.?# 2,1,1,1", 2)
        ];

        for (test_input, expected_output) in tests {
            let record = parse_spring_condition_record(&test_input)?;

            assert_eq!(count_possible_spring_arrangements(&record), expected_output, "{} should have {} match(es)", test_input, expected_output);
        }

        Ok(())
    }

    #[test]
    fn count_possible_spring_arrangements_with_unfolded_records_returns_correct_value() -> Result<()> {
        let tests = vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 16384),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 16),
            ("????.######..#####. 1,6,5", 2500),
            ("?###???????? 3,2,1", 506250)
        ];

        for (test_input, expected_output) in tests {
            let mut record = parse_spring_condition_record(&test_input)?;
            unfold_record(&mut record, 5);

            assert_eq!(count_possible_spring_arrangements(&record), expected_output, "{} should have {} match(es)", test_input, expected_output);
        }

        Ok(())
    }
}
