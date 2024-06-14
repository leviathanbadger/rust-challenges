

#[derive(Debug, Clone, Copy)]
struct RaceDetails {
    time: u32,
    distance_record: u32
}

fn count_ways_to_win(race: RaceDetails) -> u32 {
    (1..(race.time - 1))
        .map(|push_time| (race.time - push_time) * push_time)
        .filter(|dist| *dist > race.distance_record)
        .count() as u32
}

fn day6part1(races: &[RaceDetails]) -> u32 {
    races
        .iter()
        .map(|race| count_ways_to_win(*race))
        .fold(1, |acc, next| acc * next)
}

fn main() {
    const RACES: [RaceDetails; 4] = [
        RaceDetails {
            time: 40,
            distance_record: 277
        },
        RaceDetails {
            time: 82,
            distance_record: 1338
        },
        RaceDetails {
            time: 91,
            distance_record: 1348
        },
        RaceDetails {
            time: 66,
            distance_record: 1063
        }
    ];

    println!("Day 6 part 1 answer: {}", day6part1(&RACES));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4part1_returns_correct_value() {
        const RACES: [RaceDetails; 3] = [
            RaceDetails {
                time: 7,
                distance_record: 9
            },
            RaceDetails {
                time: 15,
                distance_record: 40
            },
            RaceDetails {
                time: 30,
                distance_record: 200
            }
        ];

        assert_eq!(day6part1(&RACES), 288);
    }
}
