use std::convert::identity;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter(|c| c.is_digit(10));
            let first = iter.next().expect("first");
            let last = iter.last().unwrap_or(first);
            let high = first.to_digit(10).expect("parse first");
            let low = last.to_digit(10).expect("parse last");
            (high * 10) + low
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
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

    input
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
        .sum()
}
