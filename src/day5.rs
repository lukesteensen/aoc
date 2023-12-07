use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Input {
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

    Input { seeds, maps }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u32 {
    let Input { seeds, maps } = input;

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
    min
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u32 {
    let Input { seeds, maps } = input;

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

    min
}

#[derive(Debug)]
struct Map {
    _from: String,
    _to: String,
    ranges: Vec<Range>,
}

impl Map {
    fn new(from: &str, to: &str, ranges: Vec<Range>) -> Self {
        Self {
            _from: from.into(),
            _to: to.into(),
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
