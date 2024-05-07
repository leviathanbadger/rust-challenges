use std::fs;
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

const PART2_MAGIC_STRINGS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9)
];

fn get_calibration_value_part2(s: &str) -> Result<u32> {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    let s_bytes = s.as_bytes();
    for idx in 0..s_bytes.len() {
        let mut dig: Option<u32> = None;

        if s_bytes[idx] >= 0x31 && s_bytes[idx] <= 0x39 {
            dig = Some((s_bytes[idx] - 0x30) as u32);
        }

        if dig.is_none() {
            for (magic_str, magic_val) in PART2_MAGIC_STRINGS {
                if idx + magic_str.len() > s_bytes.len() {
                    continue;
                }
                let magic_str_bytes = magic_str.as_bytes();
                let mut is_match = true;
                for midx in 0..magic_str_bytes.len() {
                    if magic_str_bytes[midx] != s_bytes[idx + midx] {
                        is_match = false;
                        break;
                    }
                }
                if is_match {
                    dig = Some(magic_val);
                    break;
                }
            }
        }

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

fn day1part1(input: &String) -> u32 {
    input
        .split("\n")
        .map(get_calibration_value)
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .sum::<u32>()
}

fn day1part2(input: &String) -> u32 {
    input
        .split("\n")
        .map(get_calibration_value_part2)
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .sum::<u32>()
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");

    println!("Day 1 part 1 answer: {}", day1part1(&input));
    println!("Day 1 part 2 answer: {}", day1part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_calibration_value_returns_correct_values() -> Result<()> {
        assert_eq!(get_calibration_value("1abc2")?, 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx")?, 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f")?, 15);
        assert_eq!(get_calibration_value("treb7uchet")?, 77);

        Ok(())
    }

    #[test]
    fn day1part1_returns_correct_value() {
        const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(day1part1(&INPUT.to_owned()), 142);
    }

    #[test]
    fn get_calibration_value_part2_returns_correct_values() -> Result<()> {
        assert_eq!(get_calibration_value_part2("two1nine")?, 29);
        assert_eq!(get_calibration_value_part2("eightwothree")?, 83);
        assert_eq!(get_calibration_value_part2("abcone2threexyz")?, 13);
        assert_eq!(get_calibration_value_part2("xtwone3four")?, 24);
        assert_eq!(get_calibration_value_part2("4nineeightseven2")?, 42);
        assert_eq!(get_calibration_value_part2("zoneight234")?, 14);
        assert_eq!(get_calibration_value_part2("7pqrstsixteen")?, 76);

        Ok(())
    }

    #[test]
    fn day1part2_returns_correct_value() {
        const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(day1part2(&INPUT.to_owned()), 281);
    }
}
