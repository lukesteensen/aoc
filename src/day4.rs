use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (card, rest) = line.split_once(':').expect("card");
            let card_num = card.split_whitespace().nth(1).expect("card num");
            let _card_num: u32 = card_num.parse().expect("parse card num");

            let (winning, have) = rest.split_once('|').expect("rest");
            let winning: HashSet<u32> = winning
                .split_whitespace()
                .map(|s| s.parse().expect("parse winning"))
                .collect();
            let have: HashSet<u32> = have
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
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let mut copies = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let (card, rest) = line.split_once(':').expect("card");
        let card_num = card.split_whitespace().nth(1).expect("card num");
        let _card_num: u32 = card_num.parse().expect("parse card num");

        let (winning, have) = rest.split_once('|').expect("rest");
        let winning: HashSet<u32> = winning
            .split_whitespace()
            .map(|s| s.parse().expect("parse winning"))
            .collect();
        let have: HashSet<u32> = have
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
    copies.iter().sum()
}
