

#[derive(Debug, Clone, Copy)]
struct RaceDetails {
    time: u64,
    distance_record: u64
}

fn count_ways_to_win(race: RaceDetails) -> u64 {
    (1..(race.time - 1))
        .map(|push_time| (race.time - push_time) * push_time)
        .filter(|dist| *dist > race.distance_record)
        .count() as u64
}

fn count_ways_to_win_all_races(races: &[RaceDetails]) -> u64 {
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

    println!("Day 6 part 1 answer: {}", count_ways_to_win_all_races(&RACES));

    const BIG_RACE: RaceDetails = RaceDetails {
        time: 40829166,
        distance_record: 277133813481063
    };
    println!("Day 6 part 2 answer: {}", count_ways_to_win_all_races(&[BIG_RACE]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_ways_to_win_all_races_returns_correct_value() {
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

        assert_eq!(count_ways_to_win_all_races(&RACES), 288);
    }

    #[test]
    fn count_ways_to_win_all_races_returns_correct_value_2() {
        const BIG_RACE: RaceDetails = RaceDetails {
            time: 71530,
            distance_record: 940200
        };

        assert_eq!(count_ways_to_win_all_races(&[BIG_RACE]), 71503);
    }
}
