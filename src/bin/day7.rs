use std::cmp::Ordering;
use std::collections::HashMap;
use advent_of_code::{read_file_into_strings};
use std::iter::Iterator;
use std::str::FromStr;
use itertools::{enumerate, Itertools};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
}

impl Card {
    fn from_char(character: char) -> Card {
        return match character {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            '1' => Card::One,
            _ => panic!("Couldn't parse Card")
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}


#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType
}

fn get_hand_type(cards: &Vec<Card>) -> HandType {
    let mut map = HashMap::new();
    for card in cards {
        map.entry(card).and_modify(|count| *count += 1).or_insert(1);
    }
    return match map.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            for nr_of_one_card in map.values() {
               if nr_of_one_card == &4 {
                    return HandType::FourOfAKind
               }
            }
            return HandType::FullHouse
        },
        3 => {
            for nr_of_one_card in map.values() {
                if nr_of_one_card == &3 {
                    return HandType::ThreeOfAKind
                }
            }
            return HandType::TwoPair
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("What the hell!")
    }
}

impl Hand {
    fn from_str(row: &String) -> Hand {
        let parts: Vec<&str> = row.split(" ").collect();
        let cards = parts.first().unwrap().chars().map(Card::from_char).collect();
        let hand_type = get_hand_type(&cards);

        Hand {
            cards,
            bid: parts.last().unwrap().parse().unwrap(),
            hand_type
        }
    }

}

fn comparator(a: &Hand, b: &Hand) -> Ordering {
    if &a.hand_type == &b.hand_type {
        for i in 0..=4 {
            if &a.cards[i] != &b.cards[i] {
                return (a.cards[i] as isize).cmp(&(b.cards[i] as isize))
            }
        }
        return Ordering::Equal;
    }
    (a.hand_type as isize).cmp(&(b.hand_type as isize))
}

fn main() {
    let lines = read_file_into_strings("./src/input/day7/input.txt");
    let hands: Vec<Hand> = lines.iter().map(Hand::from_str).sorted_unstable_by(comparator).collect();

    let mut res: u32 = 0;

    for (idx, hand) in enumerate(hands) {
        res += hand.bid * (idx as u32 +1)
    }
    println!("{}", res)
}
