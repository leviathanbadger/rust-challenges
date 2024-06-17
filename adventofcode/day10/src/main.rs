use core::result::Result::Ok;
use std::{collections::{HashMap, VecDeque}, fs};
use anyhow::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    Ground = b'.',
    StartPosition = b'S',
    VerticalPipe = b'|',
    HorizontalPipe = b'-',
    NorthEastPipe = b'L',
    NorthWestPipe = b'J',
    SouthWestPipe = b'7',
    SouthEastPipe = b'F'
}

impl TryFrom<u8> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Tile::Ground),
            b'S' => Ok(Tile::StartPosition),
            b'|' => Ok(Tile::VerticalPipe),
            b'-' => Ok(Tile::HorizontalPipe),
            b'L' => Ok(Tile::NorthEastPipe),
            b'J' => Ok(Tile::NorthWestPipe),
            b'7' => Ok(Tile::SouthWestPipe),
            b'F' => Ok(Tile::SouthEastPipe),
            _ => Err(anyhow!("Could not convert byte to Tile"))
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>
}

impl Map {
    fn get_tile(&self, x: i32, y: i32) -> Tile {
        if y < 0 || y >= self.tiles.len() as i32 || x < 0 || x >= self.tiles[y as usize].len() as i32 {
            Tile::Ground
        }
        else {
            self.tiles[y as usize][x as usize]
        }
    }

    fn get_start_position(&self) -> Option<(i32, i32)> {
        for (row, row_tiles) in self.tiles.iter().enumerate() {
            for (col, col_tile) in row_tiles.iter().enumerate() {
                if *col_tile == Tile::StartPosition {
                    return Some((col as i32, row as i32));
                }
            }
        }
        None
    }
}

impl Tile {
    fn is_self_connected_east(self) -> bool {
        match self {
            Tile::HorizontalPipe |
            Tile::NorthEastPipe |
            Tile::SouthEastPipe => true,
            _ => false
        }
    }

    fn is_self_connected_west(self) -> bool {
        match self {
            Tile::HorizontalPipe |
            Tile::NorthWestPipe |
            Tile::SouthWestPipe => true,
            _ => false
        }
    }

    fn is_self_connected_north(self) -> bool {
        match self {
            Tile::VerticalPipe |
            Tile::NorthEastPipe |
            Tile::NorthWestPipe => true,
            _ => false
        }
    }

    fn is_self_connected_south(self) -> bool {
        match self {
            Tile::VerticalPipe |
            Tile::SouthEastPipe |
            Tile::SouthWestPipe => true,
            _ => false
        }
    }

    fn get_connected_pipes(self, x: i32, y: i32, map: &Map) -> Option<((i32, i32), (i32, i32))> {
        match self {
            Tile::Ground => None,
            Tile::HorizontalPipe => Some(((x - 1, y), (x + 1, y))),
            Tile::VerticalPipe => Some(((x, y - 1), (x, y + 1))),
            Tile::NorthEastPipe => Some(((x, y - 1), (x + 1, y))),
            Tile::NorthWestPipe => Some(((x, y - 1), (x - 1, y))),
            Tile::SouthWestPipe => Some(((x, y + 1), (x - 1, y))),
            Tile::SouthEastPipe => Some(((x, y + 1), (x + 1, y))),
            Tile::StartPosition => {
                let mut connected = vec![];
                if map.get_tile(x + 1, y).is_self_connected_west() {
                    connected.push((x + 1, y));
                }
                if map.get_tile(x - 1, y).is_self_connected_east() {
                    connected.push((x - 1, y));
                }
                if map.get_tile(x, y + 1).is_self_connected_north() {
                    connected.push((x, y + 1));
                }
                if map.get_tile(x, y - 1).is_self_connected_south() {
                    connected.push((x, y - 1));
                }
                if connected.len() != 2 {
                    None
                }
                else {
                    Some((connected[0], connected[1]))
                }
            }
        }
    }
}

fn parse_map(input: &String) -> Result<Map> {
    let tiles = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes()
            .iter()
            .map(|bit| Tile::try_from(*bit))
            .collect::<Result<Vec<Tile>>>())
        .collect::<Result<Vec<Vec<Tile>>>>()?;

    Ok(Map {
        tiles
    })
}

fn day10part1(map: &Map) -> Result<usize> {
    let start_pos = map.get_start_position().ok_or(anyhow!("Could not find start position"))?;

    let mut to_check = VecDeque::<((i32, i32), usize)>::new();
    to_check.push_back((start_pos, 0_usize));
    let mut dist_map = HashMap::<(i32, i32), usize>::new();

    while !to_check.is_empty() {
        let (pos, dist) = to_check.pop_front().unwrap();
        let was_checked = dist_map.contains_key(&pos);
        if *dist_map.get(&pos).unwrap_or(&usize::MAX) > dist {
            dist_map.insert(pos, dist);

            if !was_checked {
                let tile = map.get_tile(pos.0, pos.1);
                if let Some((rol, ror)) = tile.get_connected_pipes(pos.0, pos.1, map) {
                    to_check.push_back((rol, dist + 1));
                    to_check.push_back((ror, dist + 1));
                }
            }
        }
    }

    Ok(*dist_map.values().max().unwrap())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    let map = parse_map(&input)?;

    println!("Day 10 part 1 answer: {}", day10part1(&map)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8part1_returns_correct_value() -> Result<()> {
        const INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let map = parse_map(&INPUT.to_owned())?;

        assert_eq!(day10part1(&map)?, 4);

        Ok(())
    }

    #[test]
    fn day8part1_returns_correct_value_2() -> Result<()> {
        const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let map = parse_map(&INPUT.to_owned())?;

        assert_eq!(day10part1(&map)?, 8);

        Ok(())
    }
}
