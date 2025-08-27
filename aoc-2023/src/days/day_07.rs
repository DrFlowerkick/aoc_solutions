//!day_07.rs

use anyhow::Result;
use std::cmp::Ordering;

// Playing Card definitions
trait PlayingCard {
    fn num_card_types(&self) -> usize {
        13
    }
    fn index(&self) -> usize;
    fn joker_index(&self) -> Option<usize> {
        None
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
enum NoJokers {
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl PlayingCard for NoJokers {
    fn index(&self) -> usize {
        match self {
            NoJokers::Two => 0,
            NoJokers::Three => 1,
            NoJokers::Four => 2,
            NoJokers::Five => 3,
            NoJokers::Six => 4,
            NoJokers::Seven => 5,
            NoJokers::Eight => 6,
            NoJokers::Nine => 7,
            NoJokers::Ten => 8,
            NoJokers::Jack => 9,
            NoJokers::Queen => 10,
            NoJokers::King => 11,
            NoJokers::Ace => 12,
        }
    }
}

impl From<char> for NoJokers {
    fn from(value: char) -> Self {
        match value {
            '2' => NoJokers::Two,
            '3' => NoJokers::Three,
            '4' => NoJokers::Four,
            '5' => NoJokers::Five,
            '6' => NoJokers::Six,
            '7' => NoJokers::Seven,
            '8' => NoJokers::Eight,
            '9' => NoJokers::Nine,
            'T' => NoJokers::Ten,
            'J' => NoJokers::Jack,
            'Q' => NoJokers::Queen,
            'K' => NoJokers::King,
            'A' => NoJokers::Ace,
            _ => panic!("invalid card char"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
enum HasJokers {
    Joker,
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl PlayingCard for HasJokers {
    fn index(&self) -> usize {
        match self {
            HasJokers::Joker => 0,
            HasJokers::Two => 1,
            HasJokers::Three => 2,
            HasJokers::Four => 3,
            HasJokers::Five => 4,
            HasJokers::Six => 5,
            HasJokers::Seven => 6,
            HasJokers::Eight => 7,
            HasJokers::Nine => 8,
            HasJokers::Ten => 9,
            HasJokers::Queen => 10,
            HasJokers::King => 11,
            HasJokers::Ace => 12,
        }
    }
    fn joker_index(&self) -> Option<usize> {
        Some(0)
    }
}

impl From<char> for HasJokers {
    fn from(value: char) -> Self {
        match value {
            'J' => HasJokers::Joker,
            '2' => HasJokers::Two,
            '3' => HasJokers::Three,
            '4' => HasJokers::Four,
            '5' => HasJokers::Five,
            '6' => HasJokers::Six,
            '7' => HasJokers::Seven,
            '8' => HasJokers::Eight,
            '9' => HasJokers::Nine,
            'T' => HasJokers::Ten,
            'Q' => HasJokers::Queen,
            'K' => HasJokers::King,
            'A' => HasJokers::Ace,
            _ => panic!("invalid card char"),
        }
    }
}

// hand deginitions

#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
enum HandType {
    #[default]
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_hand<C>(hand: &[C; 5]) -> Self
    where
        C: PlayingCard + PartialEq + Eq + PartialOrd + Ord + Default,
    {
        let mut card_count: Vec<u8> = vec![0; hand[0].num_card_types()];
        for card in hand.iter() {
            card_count[card.index()] += 1;
        }

        match hand[0].joker_index() {
            Some(ji) => {
                if card_count[ji] == 0 {
                    HandType::no_jokers_in_hand(&card_count)
                } else {
                    let num_jokers = card_count.remove(ji);
                    match num_jokers {
                        5 | 4 => HandType::FiveOfAKind,
                        3 => match HandType::no_jokers_in_hand(&card_count) {
                            HandType::OnePair => HandType::FiveOfAKind,
                            HandType::HighCard => HandType::FourOfAKind,
                            _ => panic!("invalid hand type"),
                        },
                        2 => match HandType::no_jokers_in_hand(&card_count) {
                            HandType::ThreeOfAKind => HandType::FiveOfAKind,
                            HandType::OnePair => HandType::FourOfAKind,
                            HandType::HighCard => HandType::ThreeOfAKind,
                            _ => panic!("invalid hand type"),
                        },
                        1 => match HandType::no_jokers_in_hand(&card_count) {
                            HandType::FourOfAKind => HandType::FiveOfAKind,
                            HandType::ThreeOfAKind => HandType::FourOfAKind,
                            HandType::TwoPair => HandType::FullHouse,
                            HandType::OnePair => HandType::ThreeOfAKind,
                            HandType::HighCard => HandType::OnePair,
                            _ => panic!("invalid hand type"),
                        },
                        _ => panic!("invalid number of jokers"),
                    }
                }
            }
            None => HandType::no_jokers_in_hand(&card_count),
        }
    }
    fn no_jokers_in_hand(card_count: &[u8]) -> Self {
        if card_count.contains(&5) {
            HandType::FiveOfAKind
        } else if card_count.contains(&4) {
            HandType::FourOfAKind
        } else if card_count.contains(&3) && card_count.contains(&2) {
            HandType::FullHouse
        } else if card_count.contains(&3) {
            HandType::ThreeOfAKind
        } else if card_count.iter().filter(|n| **n == 2).count() == 2 {
            HandType::TwoPair
        } else if card_count.contains(&2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Default, Eq)]
struct CardHand<C: PlayingCard + PartialEq + Eq + PartialOrd + Ord + Default + From<char>> {
    hand: [C; 5],
    bid: u64,
    hand_type: HandType,
}

impl<C: PlayingCard + PartialEq + Eq + PartialOrd + Ord + Default + From<char>> PartialEq
    for CardHand<C>
{
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl<C: PlayingCard + PartialEq + Eq + PartialOrd + Ord + Default + From<char>> Ord
    for CardHand<C>
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (s, o) in self.hand.iter().zip(other.hand.iter()) {
                    match s.cmp(o) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => (),
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl<C: PlayingCard + PartialEq + Eq + PartialOrd + Ord + Default + From<char>> PartialOrd
    for CardHand<C>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: PlayingCard + PartialEq + Eq + PartialOrd + Ord + Default + From<char>> CardHand<C> {
    fn new(hand: &str, bid: u64) -> Self {
        if hand.chars().count() != 5 {
            panic!("invalid hand");
        }
        let mut result = Self::default();
        for (i, c) in hand.chars().map(|c| C::from(c)).enumerate() {
            result.hand[i] = c;
        }
        result.bid = bid;
        result.hand_type = HandType::from_hand(&result.hand);

        result
    }
}

pub fn day_07() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_07.txt");
    // part 1
    let mut card_hands: Vec<CardHand<NoJokers>> = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            CardHand::new(hand, bid.parse::<u64>().unwrap())
        })
        .collect();
    card_hands.sort();

    let mut result_part1: u64 = 0;
    for (rank, bid) in card_hands
        .iter()
        .enumerate()
        .map(|(r, b)| (1_u64 + r as u64, b.bid))
    {
        result_part1 += rank * bid;
    }
    println!("result day 07 part 1: {}", result_part1);
    assert_eq!(result_part1, 251_927_063);

    // part 2
    let mut card_hands: Vec<CardHand<HasJokers>> = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            CardHand::new(hand, bid.parse::<u64>().unwrap())
        })
        .collect();
    card_hands.sort();

    let mut result_part2: u64 = 0;
    for (rank, bid) in card_hands
        .iter()
        .enumerate()
        .map(|(r, b)| (1_u64 + r as u64, b.bid))
    {
        result_part2 += rank * bid;
    }
    println!("result day 07 part 2: {}", result_part2);
    assert_eq!(result_part2, 255_632_664);

    Ok(())
}
