use itertools::Itertools;
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use CardVal::{Court, Num};
use PokerHand::{
    Flush, Four, FullHouse, HighCard, OnePair, Straight, StraightFlush, Three, TwoPair,
};
/**
Given a list of poker hands, return a list of those hands which win.

Note the type signature: this function should return _the same_ reference to
the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
*/
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut card_vec: Vec<&'a str> = hands.to_vec();
    card_vec.sort_by(|a, b| {
        Hand::from_str(a)
            .expect("invalid hand")
            .partial_cmp(&Hand::from_str(b).expect("invalid hand"))
            .unwrap_or(Ordering::Equal)
    });

    if card_vec.is_empty() {
        None
    } else {
        Some(
            card_vec
                .iter()
                .rev()
                .take_while(|x| {
                    Hand::from_str(x)
                        .unwrap()
                        .partial_cmp(&Hand::from_str(card_vec.iter().last().unwrap()).unwrap())
                        .unwrap_or(Ordering::Equal)
                        == Ordering::Equal
                })
                .copied()
                .collect::<Vec<&'a str>>(),
        )
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Suit {
    S,
    D,
    C,
    H,
}

impl std::str::FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Suit::S),
            "D" => Ok(Suit::D),
            "C" => Ok(Suit::C),
            "H" => Ok(Suit::H),
            _ => Err(format!("'{}' is not a valid value for Suit", s)),
        }
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
enum Face {
    J,
    Q,
    K,
    A,
}
impl std::str::FromStr for Face {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "J" => Ok(Face::J),
            "Q" => Ok(Face::Q),
            "K" => Ok(Face::K),
            "A" => Ok(Face::A),
            _ => Err(format!("'{}' is not a valid value for Face", s)),
        }
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
enum CardVal {
    Num(i32),
    Court(Face),
}

impl std::str::FromStr for CardVal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Face::from_str(s) {
            Ok(face) => Ok(Court(face)),
            _ => match i32::from_str(s) {
                Ok(i) if (1..11).contains(&i) => Ok(CardVal::Num(i)),
                _ => Err(format!("'{}' is not a valid value for CardVal", s)),
            },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Card {
    suit: Suit,
    value: CardVal,
}
impl std::str::FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.len();
        let (value_str, suit_str) = s.split_at(length - 1);
        let suit_card = Suit::from_str(suit_str)?;
        let value_card = CardVal::from_str(value_str)?;
        Ok(Card {
            suit: suit_card,
            value: value_card,
        })
    }
}
#[derive(PartialEq, Eq, Debug)]
struct Hand(HashSet<Card>);
use std::str::FromStr;
impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: HashSet<Card> = s
            .split_whitespace()
            .map(|substr| Card::from_str(substr).expect("cannot be parsed as Card"))
            .collect();

        Ok(Hand(cards))
    }
}
impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let pokerhands = check_hand(self).partial_cmp(&check_hand(other));
        match pokerhands {
            None => None,
            Some(Ordering::Equal) => {
                rank_frequencies_values(self).partial_cmp(&rank_frequencies_values(other))
            }
            Some(ord) => Some(ord),
        }
    }
}
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    Straight,
    Flush,
    FullHouse,
    Four,
    StraightFlush,
}

fn check_hand(hand: &Hand) -> PokerHand {
    if num_suits(hand) == 1 && consecutive_cards(hand) == 5 {
        StraightFlush
    } else if highest_frequency(hand) == 4 {
        Four
    } else if highest_frequency(hand) == 3 && num_pairs(hand) == 1 {
        FullHouse
    } else if num_suits(hand) == 1 {
        Flush
    } else if consecutive_cards(hand) == 5 {
        Straight
    } else if highest_frequency(hand) == 3 {
        Three
    } else if num_pairs(hand) == 2 {
        TwoPair
    } else if num_pairs(hand) == 1 {
        OnePair
    } else {
        HighCard
    }
}

fn num_suits(hand: &Hand) -> usize {
    let Hand(hashset) = hand;
    hashset.iter().map(|card| card.suit).unique().count()
}

fn card_value(card: &Card) -> i32 {
    match card.value {
        Num(i) => i,
        Court(Face::J) => 11,
        Court(Face::Q) => 12,
        Court(Face::K) => 13,
        Court(Face::A) => 14,
    }
}
fn card_value_low_a(card: &Card) -> i32 {
    match card.value {
        Court(Face::A) => 1,
        _ => card_value(card),
    }
}
fn consecutive_cards(hand: &Hand) -> usize {
    let Hand(hashset) = hand;
    let (num, _) = hashset
        .iter()
        .map(|card| card_value(card))
        .sorted()
        .enumerate()
        .fold1(|(j, acc), (i, x)| if x == (acc + 1) { (i, x) } else { (j, acc) })
        .unwrap_or((0, 0));
    let (num_low, _) = hashset
        .iter()
        .map(|card| card_value_low_a(card))
        .sorted()
        .enumerate()
        .fold1(|(j, acc), (i, x)| if x == (acc + 1) { (i, x) } else { (j, acc) })
        .unwrap_or((0, 0));
    cmp::max(num + 1, num_low + 1)
}
fn number_frequencies(hand: &Hand) -> HashMap<i32, usize> {
    let Hand(hashset) = hand;
    let mut map = HashMap::new();
    for card in hashset.iter() {
        *map.entry(card_value(card)).or_default() += 1;
    }
    map
}
fn highest_frequency(hand: &Hand) -> usize {
    let frequencies = number_frequencies(hand);
    *frequencies.values().max().unwrap_or(&0)
}
fn num_pairs(hand: &Hand) -> usize {
    let frequencies = number_frequencies(hand);
    frequencies.values().filter(|&x| *x == 2).count()
}
fn values_multiples(hand: &Hand, multiples: usize) -> Vec<i32> {
    let frequencies = number_frequencies(hand);
    let cardval_vec: Vec<i32> = frequencies
        .into_iter()
        .filter(|(_key, val)| *val == multiples)
        .map(|x| x.0)
        .sorted_unstable()
        .rev()
        .collect();
    cardval_vec
}

fn sorted_cards_straight(hand: &Hand) -> Vec<i32> {
    let Hand(hashset) = hand;
    let sorted_values: Vec<i32> = hashset
        .iter()
        .map(|card| card_value(card))
        .sorted_unstable()
        .rev()
        .collect();
    let sorted_values_low_a: Vec<i32> = hashset
        .iter()
        .map(|card| card_value_low_a(card))
        .sorted_unstable()
        .rev()
        .collect();
    if sorted_values.contains(&2) {
        sorted_values_low_a
    } else {
        sorted_values
    }
}

fn rank_frequencies_values(hand: &Hand) -> Vec<i32> {
    match consecutive_cards(hand) {
        5 => sorted_cards_straight(hand),
        _ => [
            values_multiples(hand, 5),
            values_multiples(hand, 4),
            values_multiples(hand, 3),
            values_multiples(hand, 2),
            values_multiples(hand, 1),
        ]
        .concat(),
    }
}
