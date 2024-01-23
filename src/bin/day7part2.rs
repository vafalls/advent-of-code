use std::cmp::Ordering;
use std::collections::HashMap;
use advent_of_code::{read_file_into_strings};
use std::iter::Iterator;
use itertools::{enumerate, Itertools};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Ten = 11,
    Nine = 10,
    Eight = 9,
    Seven = 8,
    Six = 7,
    Five = 6,
    Four = 5,
    Three = 4,
    Two = 3,
    One = 2,
    Joker = 1,
}

impl Card {
    fn from_char(character: char) -> Card {
        return match character {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
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
            'J' => Card::Joker,
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

    manipulate_with_jokers(cards, &mut map);

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

fn manipulate_with_jokers<'a>(cards: &'a Vec<Card>, map: &mut HashMap<&'a Card, i32>) {
    let mut jokers: Vec<Card> = Vec::new();
    for card in cards {
        if card == &Card::Joker {
            jokers.push(*card);
        } else {
            map.entry(card).and_modify(|count| *count += 1).or_insert(1);
        }
    }
    for _ in &jokers {
        if let Some(mvp_card_entry) = map.iter().sorted_unstable_by(|c1, c2| (*c1.1 as isize).cmp(&(*c2.1 as isize))).last() {
            map.entry(mvp_card_entry.0).and_modify(|count| *count += 1);
        } else if jokers.len() == 5 {
            map.insert(&Card::Ace, 5);
        }
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
