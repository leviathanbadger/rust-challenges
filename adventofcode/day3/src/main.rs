use std::fs;

struct EngineParser<'a> {
    line_bytes: &'a Vec<&'a [u8]>,
    current_line_idx: usize,
    current_idx: usize
}

impl<'a> EngineParser<'a> {
    fn create(line_bytes: &'a Vec<&'a [u8]>) -> Self {
        EngineParser {
            line_bytes: line_bytes,
            current_line_idx: 0,
            current_idx: 0
        }
    }
}

impl<'a> Iterator for EngineParser<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_line_idx < self.line_bytes.len() {
            let line = self.line_bytes[self.current_line_idx];
            let mut start_idx: Option<usize> = None;
            let mut num_accum = 0u32;

            while self.current_idx <= line.len() {
                let bit_idx = self.current_idx;
                let bit = if bit_idx >= line.len() { 0x2e } else { line[bit_idx] };
                self.current_idx += 1;
                if bit >= 0x30 && bit <= 0x39 {
                    if start_idx == None {
                        start_idx = Some(bit_idx);
                        num_accum = 0;
                    }
                    num_accum *= 10;
                    num_accum += (bit - 0x30) as u32;
                }
                else {
                    if start_idx.is_some() {
                        for q in (i32::max(self.current_line_idx as i32 - 1, 0) as usize)..usize::min(self.current_line_idx + 2, self.line_bytes.len()) {
                            for w in (i32::max(start_idx.unwrap() as i32 - 1, 0) as usize)..usize::min(bit_idx + 1, line.len()) {
                                let bit = self.line_bytes[q][w];
                                if (bit < 0x30 || bit > 0x39) && bit != 0x2e && bit != 0x20 && bit != 0x0D && bit != 0x0A {
                                    return Some(num_accum);
                                }
                            }
                        }
                    }
                    start_idx = None;
                }
            }

            self.current_line_idx += 1;
            self.current_idx = 0;
        }

        None
    }
}

fn day2part1(input: &String) -> u32 {
    let line_bytes = input
        .split("\n")
        .map(|line| line.as_bytes())
        .filter(|line_bytes| line_bytes.len() > 0)
        .collect::<Vec<&[u8]>>();

    EngineParser::create(&line_bytes)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");

    println!("Day 3 part 1 answer: {}", day2part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3part1_returns_correct_value() {
        const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(day2part1(&INPUT.to_owned()), 4361);
    }

    #[test]
    fn day3part1_returns_correct_value_2() {
        const INPUT: &str = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";
        assert_eq!(day2part1(&INPUT.to_owned()), 413);
    }

    #[test]
    fn day3part1_returns_correct_value_3() {
        const INPUT: &str = "12.......*..
12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        assert_eq!(day2part1(&INPUT.to_owned()), 925);
    }

    #[test]
    fn day3part1_returns_correct_value_4() {
        const INPUT: &str = ".......5......
..7*..*.......
...*13*.......
.......15.....";
        assert_eq!(day2part1(&INPUT.to_owned()), 40);
    }

    #[test]
    fn day3part1_returns_correct_value_5() {
        const INPUT: &str = "........
.24..4..
......*.";
        assert_eq!(day2part1(&INPUT.to_owned()), 4);
    }

    #[test]
    fn day3part1_returns_correct_value_6() {
        const INPUT: &str = "100
200";
        assert_eq!(day2part1(&INPUT.to_owned()), 0);
    }

    #[test]
    fn day3part1_returns_correct_value_7() {
        const INPUT: &str = "416.........................559...............417...............785.......900.......284...........503...796....992..........................
.........702*....772............378..569.........&.49..606...14*..............$.453*.........307....*......$.....-.................995......
.....................458...856......+.........+....&..............680.......104.............%....516.................................*......";
        assert_eq!(day2part1(&INPUT.to_owned()), 7486);
    }

    #[test]
    fn engine_parser_returns_correct_values() {
        const INPUT: &str = "416.........................559...............417...............785.......900.......284...........503...796....992..........................
.........702*....772............378..569.........&.49..606...14*..............$.453*.........307....*......$.....-.................995......
.....................458...856......+.........+....&..............680.......104.............%....516.................................*......";
        let line_bytes = INPUT
            .split("\n")
            .map(|line| line.as_bytes())
            .filter(|line_bytes| line_bytes.len() > 0)
            .collect::<Vec<&[u8]>>();

        let mut parser = EngineParser::create(&line_bytes);

        assert_eq!(parser.next(), Some(417));
        assert_eq!(parser.next(), Some(785));
        assert_eq!(parser.next(), Some(284));
        assert_eq!(parser.next(), Some(503));
        assert_eq!(parser.next(), Some(796));
        assert_eq!(parser.next(), Some(992));
        assert_eq!(parser.next(), Some(702));
        assert_eq!(parser.next(), Some(569));
        assert_eq!(parser.next(), Some(49));
        assert_eq!(parser.next(), Some(14));
        assert_eq!(parser.next(), Some(453));
        assert_eq!(parser.next(), Some(307));
        assert_eq!(parser.next(), Some(995));
        assert_eq!(parser.next(), Some(104));
        assert_eq!(parser.next(), Some(516));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn day3part1_returns_correct_value_8() {
        const INPUTS: [&str; 8] = [
            "*..
.42",
            ".*.
.42",
            "..*
42.",
            "*42",
            "42*",
            ".42
*..",
            ".42
.*.",
            "42.
..*"
        ];
        for input in INPUTS {
            assert_eq!(day2part1(&input.to_owned()), 42);
        }
    }
}
