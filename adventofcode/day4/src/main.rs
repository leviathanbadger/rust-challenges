use core::result::Result::Ok;
use std::fs;
use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Scratchcard {
    #[allow(dead_code)]
    id: u32,
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>
}

lazy_static! {
    static ref CARD_REGEX: Regex = Regex::new(r"^Card *([0-9]+): *([0-9 ]+)[|]([0-9 ]+)$").unwrap();
}

fn parse_num(str: &str) -> Result<u32> {
    Ok(str.parse::<u32>()?)
}

fn parse_card(line: &str) -> Result<Scratchcard> {
    let capture_opt = CARD_REGEX.captures(line.trim());
    match capture_opt {
        None => Err(anyhow!("Could not parse card ID from line")),
        Some(capture) => {
            let (_, [card_id_str, winning_nums_str, actual_nums_str]) = capture.extract();
            let id = card_id_str.parse::<u32>()?;
            let winning_numbers = winning_nums_str
                .split(' ')
                .filter(|num_str| !num_str.is_empty())
                .map(parse_num)
                .collect::<Result<Vec<u32>>>()?;
            let actual_numbers = actual_nums_str
                .split(' ')
                .filter(|num_str| !num_str.is_empty())
                .map(parse_num)
                .collect::<Result<Vec<u32>>>()?;
            Ok(Scratchcard {
                id,
                winning_numbers,
                actual_numbers
            })
        }
    }
}

fn parse_cards(input: &String, cards: &mut Vec<Scratchcard>) {
    for line in input.lines() {
        if let Ok(card) = parse_card(line) {
            cards.push(card);
        }
        else {
            println!("Failed to parse card! {:?}", line);
        }
    }
}

fn get_card_score(card: &Scratchcard) -> u32 {
    let num_duplicates = card.winning_numbers
        .iter()
        .map(|num| card.actual_numbers.contains(num))
        .filter(|is_match| *is_match)
        .count();
    match num_duplicates {
        0 => 0,
        _ => 2_i32.pow((num_duplicates as u32) - 1) as u32
    }
}

fn day4part1(cards: &Vec<Scratchcard>) -> u32 {
    cards.iter()
        .map(get_card_score)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    let mut cards = vec![];
    parse_cards(&input, &mut cards);

    println!("Day 4 part 1 answer: {}", day4part1(&cards));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_card_returns_correct_value() -> Result<()> {
        let card = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")?;

        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers.len(), 5);
        assert_eq!(card.winning_numbers.iter().sum::<u32>(), 41 + 48 + 83 + 86 + 17);
        assert_eq!(card.actual_numbers.len(), 8);
        assert_eq!(card.actual_numbers.iter().sum::<u32>(), 83 + 86 + 6 + 31 + 17 + 9 + 48 + 53);

        Ok(())
    }

    #[test]
    fn get_card_score_returns_correct_value() {
        let card = Scratchcard {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            actual_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
        };

        assert_eq!(get_card_score(&card), 8);
    }

    #[test]
    fn get_card_score_returns_correct_value_2() {
        let card = Scratchcard {
            id: 4,
            winning_numbers: vec![41, 92, 73, 84, 69],
            actual_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83]
        };

        assert_eq!(get_card_score(&card), 1);
    }

    #[test]
    fn get_card_score_returns_correct_value_3() {
        let card = Scratchcard {
            id: 5,
            winning_numbers: vec![87, 83, 26, 28, 32],
            actual_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36]
        };

        assert_eq!(get_card_score(&card), 0);
    }

    #[test]
    fn day4part1_returns_correct_value() {
        const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut cards = vec![];
        parse_cards(&INPUT.to_owned(), &mut cards);
        assert_eq!(day4part1(&cards), 13);
    }
}
