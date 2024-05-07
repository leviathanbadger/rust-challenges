use core::result::Result::Ok;
use std::fs;
use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(PartialEq, Eq, Default, Debug)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>
}

impl Game {
    fn is_game_possible(&self, max_set: GameSet) -> bool {
        self.sets
            .iter()
            .all(|set| set.red <= max_set.red && set.green <= max_set.green && set.blue <= max_set.blue)
    }
}

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"^Game ([0-9]+):(.*)$").unwrap();
    static ref GAME_SET_REGEX: Regex = Regex::new(r" *([0-9]+) +([a-z]+) *").unwrap();
}

fn parse_set(str: &str) -> Result<GameSet> {
    let capture_opt = GAME_SET_REGEX.captures_iter(str);
    let mut red_opt: Option<u32> = None;
    let mut green_opt: Option<u32> = None;
    let mut blue_opt: Option<u32> = None;
    for (_, [num_str, qualifier_str]) in capture_opt.map(|c| c.extract()) {
        let count = num_str.parse::<u32>()?;
        match qualifier_str {
            "red" => {
                if red_opt != None {
                    return Err(anyhow!("'red' is specified multiple times"));
                }
                red_opt = Some(count)
            },
            "green" => {
                if green_opt != None {
                    return Err(anyhow!("'green' is specified multiple times"));
                }
                green_opt = Some(count)
            },
            "blue" => {
                if blue_opt != None {
                    return Err(anyhow!("'blue' is specified multiple times"));
                }
                blue_opt = Some(count)
            },
            _ => {
                return Err(anyhow!("Unrecognized qualifier: {}", qualifier_str))
            }
        }
    }
    Ok(GameSet {
        red: red_opt.unwrap_or(0),
        green: green_opt.unwrap_or(0),
        blue: blue_opt.unwrap_or(0)
    })
}

fn parse_game(line: &str) -> Result<Game> {
    let capture_opt = GAME_REGEX.captures(line);
    match capture_opt {
        None => Err(anyhow!("Could not parse game ID from line")),
        Some(capture) => {
            let (_, [game_id_str, sets_str]) = capture.extract();
            let game_id = game_id_str.parse::<u32>()?;
            let sets = sets_str
                .split(";")
                .map(|set_str| parse_set(set_str))
                .collect::<Result<Vec<GameSet>>>()?;
            Ok(Game { id: game_id, sets: sets })
        }
    }
}

fn parse_games(input: &String, games: &mut Vec<Game>) {
    for line in input.split("\n") {
        if let Ok(game) = parse_game(line) {
            games.push(game);
        }
    }
}

const DAY2_PART1_MAX_SET: GameSet = GameSet { red: 12, green: 13, blue: 14 };

fn day2part1(input: &String) -> u32 {
    let mut games = vec!();
    parse_games(input, &mut games);
    games
        .iter()
        .filter(|game| game.is_game_possible(DAY2_PART1_MAX_SET))
        .map(|game| game.id)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");

    println!("Day 2 part 1 answer: {}", day2part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_set_returns_correct_value() -> Result<()> {
        assert_eq!(parse_set("3 blue, 4 red")?, GameSet { red: 4, green: 0, blue: 3 });
        assert_eq!(parse_set("1 red, 2 green, 6 blue")?, GameSet { red: 1, green: 2, blue: 6 });
        assert_eq!(parse_set("2 green")?, GameSet { red: 0, green: 2, blue: 0 });

        Ok(())
    }

    #[test]
    fn parse_game_returns_correct_value() -> Result<()> {
        let game = parse_game("Game 27: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?;

        assert_eq!(game.id, 27);
        assert_eq!(game.sets.len(), 3);
        assert_eq!(game.sets[0], GameSet { red: 4, green: 0, blue: 3 });
        assert_eq!(game.sets[1], GameSet { red: 1, green: 2, blue: 6 });
        assert_eq!(game.sets[2], GameSet { red: 0, green: 2, blue: 0 });

        Ok(())
    }

    #[test]
    fn day2part1_returns_correct_value() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(day2part1(&INPUT.to_owned()), 8);
    }
}
