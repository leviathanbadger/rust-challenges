use core::result::Result::Ok;
use std::{collections::{HashMap, HashSet, VecDeque}, fs};
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

fn day10part2(map: &Map) -> Result<usize> {
    let start_pos = map.get_start_position().ok_or(anyhow!("Could not find start position"))?;

    let mut to_check = VecDeque::<(i32, i32)>::new();
    to_check.push_back(start_pos);
    let mut loop_nodes = HashSet::<(i32, i32)>::new();

    let mut minx = i32::MAX;
    let mut miny = i32::MAX;
    let mut maxx = i32::MIN;
    let mut maxy = i32::MIN;

    while !to_check.is_empty() {
        let pos = to_check.pop_front().unwrap();
        if !loop_nodes.contains(&pos) {
            loop_nodes.insert(pos);

            minx = i32::min(minx, pos.0);
            miny = i32::min(miny, pos.1);
            maxx = i32::max(maxx, pos.0);
            maxy = i32::max(maxy, pos.1);

            let tile = map.get_tile(pos.0, pos.1);
            if let Some((rol, ror)) = tile.get_connected_pipes(pos.0, pos.1, map) {
                to_check.push_back(rol);
                to_check.push_back(ror);
            }
        }
    }

    let mut enclosed_tiles = 0;

    for y in miny..(maxy + 1) {
        let mut pipes_passed = 0;
        for x in minx..maxx {
            if loop_nodes.contains(&(x, y)) {
                let tile = map.get_tile(x, y);
                if tile.is_self_connected_south() {
                    pipes_passed += 1;
                }
            }
            else if pipes_passed % 2 == 1 {
                enclosed_tiles += 1;
            }
        }
    }

    Ok(enclosed_tiles)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    let map = parse_map(&input)?;

    println!("Day 10 part 1 answer: {}", day10part1(&map)?);
    println!("Day 10 part 2 answer: {}", day10part2(&map)?);

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

    #[test]
    fn day8part2_returns_correct_value() -> Result<()> {
        const INPUT: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let map = parse_map(&INPUT.to_owned())?;

        assert_eq!(day10part2(&map)?, 4);

        Ok(())
    }

    #[test]
    fn day8part2_returns_correct_value_2() -> Result<()> {
        const INPUT: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        let map = parse_map(&INPUT.to_owned())?;

        assert_eq!(day10part2(&map)?, 4);

        Ok(())
    }

    #[test]
    fn day8part2_returns_correct_value_3() -> Result<()> {
        const INPUT: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let map = parse_map(&INPUT.to_owned())?;

        assert_eq!(day10part2(&map)?, 8);

        Ok(())
    }

    #[test]
    fn day8part2_returns_correct_value_4() -> Result<()> {
        const INPUT: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let map = parse_map(&INPUT.to_owned())?;

        assert_eq!(day10part2(&map)?, 10);

        Ok(())
    }
}
