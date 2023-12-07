use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn generator(input: &str) -> HashMap<u32, HashMap<String, u32>> {
    let mut maxes_per_game: HashMap<u32, HashMap<String, u32>> = HashMap::new();
    for line in input.lines() {
        let (game, rest) = line.split_once(": ").expect("game");
        let (_game, num) = game.split_once(" ").expect("game num");
        let game_num: u32 = num.parse().expect("parse num");

        for game in rest.split(";") {
            for color in game.split(",") {
                let (count, color) = color.trim().split_once(" ").expect("split color");
                let count: u32 = count.parse().expect("parse count");
                let game_entry = maxes_per_game.entry(game_num).or_default();
                let entry = game_entry.entry(color.trim().to_string()).or_default();
                if count > *entry {
                    *entry = count;
                }
            }
        }
    }

    maxes_per_game
}

#[aoc(day2, part1)]
fn part1(maxes_per_game: &HashMap<u32, HashMap<String, u32>>) -> u32 {
    maxes_per_game
        .iter()
        .filter_map(|(key, vals)| {
            (vals["red"] <= 12 && vals["green"] <= 13 && vals["blue"] <= 14).then_some(key)
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(maxes_per_game: &HashMap<u32, HashMap<String, u32>>) -> u32 {
    maxes_per_game
        .values()
        .map(|vals| vals["red"] * vals["green"] * vals["blue"])
        .sum()
}
