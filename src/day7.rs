use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Write},
};

use aoc_runner_derive::aoc;

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut hands = input.lines().map(|line| Hand::parse(line, false)).collect::<Vec<_>>();
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = i + 1;
            hand.bid * rank
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let mut hands = input.lines().map(|line| Hand::parse(line, true)).collect::<Vec<_>>();
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = i + 1;
            hand.bid * rank
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    ty: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn parse(input: &str, with_jokers: bool) -> Self {
        let (cards, bid) = input.split_once(" ").expect("once");
        let cards = cards.chars().map(Card::from).collect::<Vec<_>>();
        assert_eq!(5, cards.len());
        let bid: usize = bid.parse().expect("bid");
        Self {
            ty: HandType::from_hand(&cards, with_jokers),
            cards: cards.try_into().unwrap(),
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order = self.ty.cmp(&other.ty);
        if order.is_ne() {
            Some(order)
        } else {
            for (s, o) in self.cards.iter().zip(other.cards.iter()) {
                let order = s.v.cmp(&o.v);
                if order.is_ne() {
                    return Some(order);
                }
            }
            panic!("equal hands");
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("bad partial_cmp")
    }
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.cards {
            f.write_char(c.c)?;
        }
        f.write_char(' ')?;
        f.write_fmt(format_args!("{:?}", self.ty))?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Card {
    c: char,
    v: u8,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        let v = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!("invalid char {c:?}"),
        };

        Self { c, v }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    fn from_hand(hand: &[Card], with_jokers: bool) -> Self {
        let mut counts: HashMap<Card, u8> = HashMap::new();
        for c in hand {
            let entry = counts.entry(*c).or_default();
            *entry += 1;
        }

        if with_jokers {
            if let Some(js) = counts.remove(&Card::from('J')) {
                let mut rest = counts
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (v, k))
                    .collect::<Vec<_>>();
                rest.sort_by_key(|(a, _)| *a);
                if let Some((_count, card)) = rest.pop() {
                    let c = counts.get_mut(&card).expect("nope");
                    *c += js;
                } else {
                    counts.insert(Card::from('J'), js);
                }
            }
        }

        let mut counts: Vec<u8> = counts.into_values().collect();
        counts.sort();
        counts.reverse();
        match &counts[..] {
            [5] => HandType::FiveOfKind,
            [4, 1] => HandType::FourOfKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("impossible: {counts:?}"),
        }
    }
}
