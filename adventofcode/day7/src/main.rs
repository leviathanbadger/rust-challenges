use core::result::Result::Ok;
use std::fs;
use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Card {
    Two = b'2',
    Three = b'3',
    Four = b'4',
    Five = b'5',
    Six = b'6',
    Seven = b'7',
    Eight = b'8',
    Nine = b'9',
    T = b'T',
    J = b'J',
    Q = b'Q',
    K = b'K',
    A = b'A'
}

impl Card {
    fn card_value(self) -> u32 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::T => 10,
            Card::J => 11,
            Card::Q => 12,
            Card::K => 13,
            Card::A => 14
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.card_value().cmp(&other.card_value())
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::convert::TryFrom<u8> for Card {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b'2' => Ok(Card::Two),
            b'3' => Ok(Card::Three),
            b'4' => Ok(Card::Four),
            b'5' => Ok(Card::Five),
            b'6' => Ok(Card::Six),
            b'7' => Ok(Card::Seven),
            b'8' => Ok(Card::Eight),
            b'9' => Ok(Card::Nine),
            b'T' => Ok(Card::T),
            b'J' => Ok(Card::J),
            b'Q' => Ok(Card::Q),
            b'K' => Ok(Card::K),
            b'A' => Ok(Card::A),
            _ => Err(anyhow!("Could not convert u8 to Card"))
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: u32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for q in 0..5 {
                if self.cards[q] != other.cards[q] {
                    return self.cards[q].cmp(&other.cards[q]);
                }
            }
            std::cmp::Ordering::Equal
        }
        else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Hand {
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

lazy_static! {
    static ref HAND_REGEX: Regex = Regex::new(r"^([2-9TJQKA]{5}) +([0-9]+)$").unwrap();
}

fn determine_hand_type(mut cards: [Card; 5]) -> HandType {
    cards.sort_unstable();

    let first = cards[0];
    let last = cards[4];

    if cards[1] == first && cards[2] == first && cards[3] == first {
        if cards[4] == first {
            return HandType::FiveOfAKind;
        }
        else {
            return HandType::FourOfAKind;
        }
    }

    if cards[1] == last && cards[2] == last && cards[3] == last {
        return HandType::FourOfAKind;
    }

    if cards[1] == first && cards[3] == last && (cards[2] == first || cards[2] == last) {
        return HandType::FullHouse;
    }

    if (cards[1] == first && cards[2] == first) || (cards[2] == last && cards[3] == last) || (cards[1] == cards[2] && cards[1] == cards[3]) {
        return HandType::ThreeOfAKind;
    }

    let mut pair_count = 0;
    for q in 0..4 {
        if cards[q] == cards[q + 1] {
            pair_count += 1;
        }
    }

    match pair_count {
        2 => HandType::TwoPair,
        1 => HandType::OnePair,
        _ => HandType::HighCard
    }
}

fn parse_hand(line: &str) -> Result<Hand> {
    let capture_opt = HAND_REGEX.captures(line);
    match capture_opt {
        None => Err(anyhow!("Could not parse hand from line: {:?}", line)),
        Some(capture) => {
            let (_, [cards_str, bid_str]) = capture.extract();
            let bid = bid_str.parse::<u32>()?;
            let cards_bits = cards_str.as_bytes();

            let mut cards = [Card::Two; 5];
            for q in 0..5 {
                cards[q] = Card::try_from(cards_bits[q])?;
            }

            let hand_type = determine_hand_type(cards);

            Ok(Hand {
                bid,
                cards,
                hand_type
            })
        }
    }
}

fn parse_hands(input: &String) -> Result<Vec<Hand>> {
    let mut hands = input.lines()
        .filter(|line| !line.is_empty())
        .map(parse_hand)
        .collect::<Result<Vec<Hand>>>()?;

    hands.sort_unstable();

    Ok(hands)
}

fn day7part1(hands: &Vec<Hand>) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * ((idx as u32) + 1))
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    let hands = parse_hands(&input)?;

    println!("Day 7 part 1 answer: {}", day7part1(&hands));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_cmp_returns_currect_value() {
        assert!(Card::A > Card::Two);
        assert!(Card::T > Card::Nine);
        assert!(Card::Seven > Card::Five);
        assert!(Card::A > Card::K);
        assert!(Card::K < Card::A);
        assert!(Card::Q > Card::T);
        assert!(Card::Five == Card::Five);
    }

    #[test]
    fn card_from_u8_returns_currect_value() -> Result<()> {
        assert_eq!(Card::try_from(b'2')?, Card::Two);
        assert_eq!(Card::try_from(b'8')?, Card::Eight);
        assert_eq!(Card::try_from(b'A')?, Card::A);
        assert_eq!(Card::try_from(b'T')?, Card::T);
        assert_eq!(Card::try_from(b'Q')?, Card::Q);

        Ok(())
    }

    #[test]
    fn hand_cmp_returns_currect_value() {
        let five_aces = [Card::A; 5];
        let five_kings = [Card::K; 5];
        assert!(Hand { bid: 2, hand_type: HandType::FiveOfAKind, cards: five_aces } == Hand { bid: 3, hand_type: HandType::FiveOfAKind, cards: five_aces });
        assert!(Hand { bid: 2, hand_type: HandType::ThreeOfAKind, cards: five_aces } < Hand { bid: 3, hand_type: HandType::FiveOfAKind, cards: five_aces });
        assert!(Hand { bid: 2, hand_type: HandType::TwoPair, cards: five_aces } < Hand { bid: 3, hand_type: HandType::FullHouse, cards: five_aces });
        assert!(Hand { bid: 2, hand_type: HandType::HighCard, cards: five_aces } < Hand { bid: 3, hand_type: HandType::FullHouse, cards: five_aces });
        assert!(Hand { bid: 2, hand_type: HandType::FiveOfAKind, cards: five_aces } > Hand { bid: 3, hand_type: HandType::FullHouse, cards: five_aces });
        assert!(Hand { bid: 2, hand_type: HandType::FiveOfAKind, cards: five_aces } > Hand { bid: 3, hand_type: HandType::FiveOfAKind, cards: five_kings });
    }

    #[test]
    fn parse_hand_returns_correct_value() -> Result<()> {
        {
            let hand = parse_hand("AAAAA 42")?;
            assert_eq!(hand.bid, 42);
            assert_eq!(hand.hand_type, HandType::FiveOfAKind);
        }

        {
            let hand = parse_hand("AAKAA 992")?;
            assert_eq!(hand.bid, 992);
            assert_eq!(hand.hand_type, HandType::FourOfAKind);
        }

        {
            let hand = parse_hand("KKAKK 1")?;
            assert_eq!(hand.bid, 1);
            assert_eq!(hand.hand_type, HandType::FourOfAKind);
        }

        {
            let hand = parse_hand("23332 2")?;
            assert_eq!(hand.bid, 2);
            assert_eq!(hand.hand_type, HandType::FullHouse);
        }

        {
            let hand = parse_hand("32223 3")?;
            assert_eq!(hand.bid, 3);
            assert_eq!(hand.hand_type, HandType::FullHouse);
        }

        {
            let hand = parse_hand("T55J5 684")?;
            assert_eq!(hand.bid, 684);
            assert_eq!(hand.hand_type, HandType::ThreeOfAKind);
        }

        {
            let hand = parse_hand("QQQJA 483")?;
            assert_eq!(hand.bid, 483);
            assert_eq!(hand.hand_type, HandType::ThreeOfAKind);
        }

        {
            let hand = parse_hand("KK677 28")?;
            assert_eq!(hand.bid, 28);
            assert_eq!(hand.hand_type, HandType::TwoPair);
        }

        {
            let hand = parse_hand("KTJJT 220")?;
            assert_eq!(hand.bid, 220);
            assert_eq!(hand.hand_type, HandType::TwoPair);
        }

        {
            let hand = parse_hand("32T3K 765")?;
            assert_eq!(hand.bid, 765);
            assert_eq!(hand.hand_type, HandType::OnePair);
        }

        {
            let hand = parse_hand("AKQJT 92")?;
            assert_eq!(hand.bid, 92);
            assert_eq!(hand.hand_type, HandType::HighCard);
        }

        Ok(())
    }

    #[test]
    fn day7part1_returns_correct_value() -> Result<()> {
        const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let hands = parse_hands(&INPUT.to_owned())?;

        assert_eq!(day7part1(&hands), 6440);

        Ok(())
    }
}
