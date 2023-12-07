use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    convert::identity, fmt::{self, Write},
};

fn main() {
    if true {
        day7();
    } else {
        day6();
        day5();
        day4();
        day3();
        day2();
        day1();
    }
}

fn day7() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let input = include_str!("../day7.input");

    let mut hands = input.lines().map(Hand::parse).collect::<Vec<_>>();
    hands.sort_unstable();

    let mut answer = 0;
    let mut rank = 0;
    let mut last = None;

    for (i, hand) in hands.iter().enumerate() {
        let position = i + 1;
        if last != Some(hand) {
            rank = position;
        } else {
            // panic!("tie");
        }
        last = Some(hand);

        println!("{position} {rank} {hand}");
        answer += hand.bid * rank;
    }

    println!("part 2: {answer}");
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    ty: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn parse(input: &str) -> Self {
        let (cards, bid) = input.split_once(" ").expect("once");
        let cards = cards.chars().map(Card::from).collect::<Vec<_>>();
        assert_eq!(5, cards.len());
        let bid: usize = bid.parse().expect("bid");
        Self {
            ty: HandType::from_hand(&cards),
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    fn from_hand(hand: &[Card]) -> Self {
        let mut counts: HashMap<Card, u8> = HashMap::new();
        for c in hand {
            let entry = counts.entry(*c).or_default();
            *entry += 1;
        }

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

fn day6() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    // let input = include_str!("../day6.input");

    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("times")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().expect("parse time"));
    let distances = lines
        .next()
        .expect("distances")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().expect("parse distance"));

    let part1_answer = times
        .zip(distances)
        .map(|(time, distance)| {
            (0..time)
                .filter_map(|t| {
                    let speed = t;
                    let d = speed * (time - t);
                    (d > distance).then(|| ())
                })
                .count()
        })
        .reduce(|x, y| x * y)
        .expect("reduce");

    println!("part 1: {part1_answer}");

    let mut lines = input.lines();
    let time = lines
        .next()
        .expect("times")
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .expect("time");
    let distance = lines
        .next()
        .expect("distances")
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .expect("distance");

    let part2_answer = (0..time)
        .filter_map(|t| {
            let speed = t;
            let d = speed * (time - t);
            (d > distance).then(|| ())
        })
        .count();

    println!("part 1: {part2_answer}");
}

fn day5() {
    let input = include_str!("../day5.input");

    let mut lines = input.lines();
    let seeds = lines
        .next()
        .expect("seeds")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().expect("seed parse"))
        .collect::<Vec<_>>();
    let _ = lines.next().expect("blank");

    let mut maps = Vec::new();
    let mut current = Vec::new();
    let mut current_keys = ("", "");
    for line in lines {
        if line.trim().is_empty() {
            maps.push(Map::new(
                current_keys.0,
                current_keys.1,
                std::mem::take(&mut current),
            ));
        } else {
            if line.chars().next().unwrap().is_ascii_alphabetic() {
                let (name, _rest) = line.split_once(' ').expect("split name");
                let (from, to) = name.split_once("-to-").expect("split from to");
                current_keys = (from, to);
            } else {
                let parsed = line
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().expect("parse ranges"))
                    .collect::<Vec<_>>();
                current.push(Range::new(parsed));
            }
        }
    }
    maps.push(Map::new(
        current_keys.0,
        current_keys.1,
        std::mem::take(&mut current),
    ));

    let mut min = u32::MAX;
    let mut end_to_seed = HashMap::new();
    for seed in seeds.iter() {
        let mut n = *seed;
        for map in maps.iter() {
            n = map.apply(n);
        }
        if n < min {
            min = n;
            end_to_seed.insert(n, seed);
        }
    }
    println!("part 1: {min}");

    let mut min = u32::MAX;
    let mut end_to_seed = HashMap::new();
    for chunk in seeds.chunks(2) {
        assert_eq!(2, chunk.len());
        let start = chunk[0];
        let len = chunk[1];
        for seed in start..(start + len) {
            let mut n = seed;
            for map in maps.iter() {
                n = map.apply(n);
            }
            if n < min {
                min = n;
                end_to_seed.insert(n, seed);
            }
        }
    }

    println!("part 2: {min}");
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Map {
    fn new(from: &str, to: &str, ranges: Vec<Range>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            ranges,
        }
    }

    fn apply(&self, n: u32) -> u32 {
        self.ranges
            .iter()
            .find_map(|range| range.apply(n))
            .unwrap_or(n)
    }
}

#[derive(Debug)]
struct Range {
    dest_start: u32,
    source_start: u32,
    length: u32,
}

impl Range {
    fn new(parsed: Vec<u32>) -> Self {
        assert_eq!(3, parsed.len());
        Self {
            dest_start: parsed[0],
            source_start: parsed[1],
            length: parsed[2],
        }
    }

    fn apply(&self, n: u32) -> Option<u32> {
        if n >= self.source_start {
            let idx = n - self.source_start;
            if idx < self.length {
                Some(self.dest_start + idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn day4() {
    let input = include_str!("../day4.input");

    let part1_answer: usize = input
        .lines()
        .map(|line| {
            let (card, rest) = line.split_once(':').expect("card");
            let card_num = card.split_whitespace().nth(1).expect("card num");
            let _card_num: u32 = card_num.parse().expect("parse card num");

            let (winning, have) = rest.split_once('|').expect("rest");
            let winning: HashSet<u32> = winning
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("parse winning"))
                .collect();
            let have: HashSet<u32> = have
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("parse have"))
                .collect();

            let matches = have.iter().filter(|n| winning.contains(n)).count() as u32;
            if matches > 0 {
                2usize.pow(matches - 1)
            } else {
                0
            }
        })
        .sum();

    println!("part 1: {part1_answer}");

    let mut copies = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let (card, rest) = line.split_once(':').expect("card");
        let card_num = card.split_whitespace().nth(1).expect("card num");
        let _card_num: u32 = card_num.parse().expect("parse card num");

        let (winning, have) = rest.split_once('|').expect("rest");
        let winning: HashSet<u32> = winning
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("parse winning"))
            .collect();
        let have: HashSet<u32> = have
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("parse have"))
            .collect();

        let matches = have.iter().filter(|n| winning.contains(n)).count();
        let card_copies = copies[i];
        // println!("card {card_num} x {card_copies}: {matches} matches");
        for x in 0..matches {
            copies[i + x + 1] += card_copies;
        }
    }
    let part2_answer: usize = copies.iter().sum();

    println!("part 2: {part2_answer}");
}

fn day3() {
    let input = include_str!("../day3.input");

    let mut grid = HashMap::new();
    let mut numbers = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut current_number = None;
        let mut current_number_coords = Vec::new();
        for (col, char) in line.char_indices() {
            grid.insert((col as i32, row as i32), char);
            if let Some(digit) = char.to_digit(10) {
                if let Some(num) = current_number {
                    current_number = Some((num * 10) + digit)
                } else {
                    current_number = Some(digit)
                }
                current_number_coords.push((col as i32, row as i32));
            } else {
                if let Some(num) = current_number.take() {
                    numbers.push((num, std::mem::take(&mut current_number_coords)));
                }
            }
        }
        if let Some(num) = current_number.take() {
            numbers.push((num, std::mem::take(&mut current_number_coords)));
        }
    }

    let part1_answer: u32 = numbers
        .iter()
        .filter_map(|(num, coords)| {
            let touches_symbol = coords
                .iter()
                .flat_map(|&(x, y)| {
                    [
                        (x, y + 1),
                        (x + 1, y + 1),
                        (x + 1, y),
                        (x + 1, y - 1),
                        (x, y - 1),
                        (x - 1, y - 1),
                        (x - 1, y),
                        (x - 1, y + 1),
                    ]
                })
                .filter(|coord| grid.contains_key(coord))
                .filter(|coord| !coords.contains(coord))
                .any(|coord| grid[&coord].is_digit(10) == false && (grid[&coord] != '.'));

            if touches_symbol {
                Some(num)
            } else {
                None
            }
        })
        .sum();

    println!("part 1: {part1_answer}");

    let mut gears: HashMap<(i32, i32), Vec<(u32, Vec<_>)>> = HashMap::new();
    for (num, coords) in numbers {
        let neighbors = coords
            .iter()
            .flat_map(|&(x, y)| {
                [
                    (x, y + 1),
                    (x + 1, y + 1),
                    (x + 1, y),
                    (x + 1, y - 1),
                    (x, y - 1),
                    (x - 1, y - 1),
                    (x - 1, y),
                    (x - 1, y + 1),
                ]
            })
            .filter(|coord| grid.contains_key(coord))
            .filter(|coord| !coords.contains(coord));
        for coord in neighbors {
            if grid[&coord] == '*' {
                let entry = gears.entry(coord).or_default();
                entry.push((num, coords.clone()));
            }
        }
    }

    for (_coord, nums) in gears.iter_mut() {
        nums.sort();
        nums.dedup();
    }

    let part2_answer: u32 = gears
        .values_mut()
        .map(|nums| nums.iter().map(|t| t.0).collect::<Vec<_>>())
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum();

    println!("part 2: {part2_answer}");
}

fn day2() {
    let input = include_str!("../day2.input");

    let mut maxes_per_game: HashMap<u32, HashMap<&str, u32>> = HashMap::new();
    for line in input.lines() {
        let (game, rest) = line.split_once(": ").expect("game");
        let (_game, num) = game.split_once(" ").expect("game num");
        let game_num: u32 = num.parse().expect("parse num");

        for game in rest.split(";") {
            for color in game.split(",") {
                let (count, color) = color.trim().split_once(" ").expect("split color");
                let count: u32 = count.parse().expect("parse count");
                let game_entry = maxes_per_game.entry(game_num).or_default();
                let entry = game_entry.entry(color.trim()).or_default();
                if count > *entry {
                    *entry = count;
                }
            }
        }
    }

    let part1_answer: u32 = maxes_per_game
        .iter()
        .filter_map(|(key, vals)| {
            (vals["red"] <= 12 && vals["green"] <= 13 && vals["blue"] <= 14).then_some(key)
        })
        .sum();

    println!("part 1: {part1_answer}");

    let part2_answer: u32 = maxes_per_game
        .values()
        .map(|vals| vals["red"] * vals["green"] * vals["blue"])
        .sum();

    println!("part 2: {part2_answer}");
}

fn day1() {
    let input = include_str!("../day1.input");

    let part1_answer: u32 = input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter(|c| c.is_digit(10));
            let first = iter.next().expect("first");
            let last = iter.last().unwrap_or(first);
            let high = first.to_digit(10).expect("parse first");
            let low = last.to_digit(10).expect("parse last");
            (high * 10) + low
        })
        .sum();

    println!("part 1: {part1_answer}");

    let patterns = [
        (["1", "one"], 1),
        (["2", "two"], 2),
        (["3", "three"], 3),
        (["4", "four"], 4),
        (["5", "five"], 5),
        (["6", "six"], 6),
        (["7", "seven"], 7),
        (["8", "eight"], 8),
        (["9", "nine"], 9),
    ];

    let part2_answer: u32 = input
        .lines()
        .map(|line| {
            let matches = patterns
                .iter()
                .flat_map(|(ps, val)| {
                    ps.iter().flat_map(|pat| {
                        let first = line.find(pat).map(|pos| (pos, *val));
                        let last = line.rfind(pat).map(|pos| (pos, *val));
                        [first, last].into_iter().filter_map(identity)
                    })
                })
                .collect::<Vec<_>>();

            let (_, high) = matches.iter().min_by_key(|(pos, _val)| pos).expect("first");
            let (_, low) = matches.iter().max_by_key(|(pos, _val)| pos).expect("last");

            (high * 10) + low
        })
        .sum();

    println!("part 2: {part2_answer}");
}
