use core::result::Result::Ok;
use std::fs;
use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Instruction {
    Left = 0,
    Right = 1
}

impl TryFrom<u8> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b'L' => Ok(Instruction::Left),
            b'R' => Ok(Instruction::Right),
            _ => Err(anyhow!("Could not parse instruction from character"))
        }
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    choice_names: [String; 2],
    choices: [usize; 2]
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: Vec<Node>,
    start_node_idx: usize,
    end_node_idx: usize
}

lazy_static! {
    static ref NODE_REGEX: Regex = Regex::new(r"^([A-Z]{3}) *= *[(]([A-Z]{3}), *([A-Z]{3})[)]$").unwrap();
}

fn parse_node(line: &str) -> Result<Node> {
    let capture_opt = NODE_REGEX.captures(line);
    match capture_opt {
        None => Err(anyhow!("Could not parse node")),
        Some(captures) => {
            let (_, [name, left_name, right_name]) = captures.extract();
            Ok(Node {
                name: name.to_owned(),
                choice_names: [left_name.to_owned(), right_name.to_owned()],
                choices: [usize::MAX; 2]
            })
        }
    }
}

fn parse_map(input: &String) -> Result<Map> {
    let mut lines = input.lines()
        .filter(|line| !line.is_empty());

    let instructions = match lines.next() {
        None => {
            return Err(anyhow!("Input string has no lines. Could not parse instructions"));
        },
        Some(instructions_str) => {
            let instructions_bits = instructions_str.as_bytes();
            instructions_bits
                .iter()
                .map(|bit| Instruction::try_from(*bit))
                .collect::<Result<Vec<Instruction>>>()?
        }
    };

    let mut nodes = lines
        .map(|line| parse_node(line))
        .collect::<Result<Vec<Node>>>()?;

    for q in 0..nodes.len() {
        let name = nodes[q].name.to_owned();
        for w in 0..nodes.len() {
            if nodes[w].choice_names[0] == *name {
                nodes[w].choices[0] = q;
            }
            if nodes[w].choice_names[1] == *name {
                nodes[w].choices[1] = q;
            }
        }
    }

    for q in 0..nodes.len() {
        if nodes[q].choices[0] == usize::MAX {
            return Err(anyhow!("Could not find node with name '{:?}'", nodes[q].choice_names[0]));
        }
        if nodes[q].choices[1] == usize::MAX {
            return Err(anyhow!("Could not find node with name '{:?}'", nodes[q].choice_names[1]));
        }
    }

    let start_node_idx = match nodes.iter().position(|node| node.name == "AAA") {
        None => {
            return Err(anyhow!("No parsed node is named AAA. Could not find start node"))
        },
        Some(idx) => idx
    };

    let end_node_idx = match nodes.iter().position(|node| node.name == "ZZZ") {
        None => {
            return Err(anyhow!("No parsed node is named ZZZ. Could not find end node"))
        },
        Some(idx) => idx
    };

    Ok(Map {
        instructions,
        nodes,
        start_node_idx,
        end_node_idx
    })
}

fn day8part1(map: &Map) -> u32 {
    let mut idx = map.start_node_idx;
    let mut step_count = 0;

    while idx != map.end_node_idx {
        let next_instruction = map.instructions[step_count % map.instructions.len()];
        idx = map.nodes[idx].choices[next_instruction as usize];
        step_count += 1;
    }

    step_count as u32
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    let map = parse_map(&input)?;

    println!("Day 8 part 1 answer: {}", day8part1(&map));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8part1_returns_correct_value() -> Result<()> {
        const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let map = parse_map(&INPUT.to_owned())?;

        assert_eq!(day8part1(&map), 2);

        Ok(())
    }

    #[test]
    fn day8part1_returns_correct_value_2() -> Result<()> {
        const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let map = parse_map(&INPUT.to_owned())?;

        assert_eq!(day8part1(&map), 6);

        Ok(())
    }
}
