use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::IndexMap;
use itertools::Itertools;

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    // print(input);
    let mut flipped = transpose(input.to_vec());
    // println!("{}", "---".repeat(10));
    // print(&flipped);
    // println!("{}", "---".repeat(10));

    let mut cache: HashMap<Vec<char>, Vec<char>> = HashMap::new();
    tilt(&mut flipped, &mut cache);

    let again = transpose(flipped);
    // println!("{}", "---".repeat(10));
    // print(&again);
    let scores = (1..=again.len()).rev();
    again
        .into_iter()
        .zip(scores)
        .map(|(row, score)| row.iter().filter(|&c| *c == 'O').count() * score)
        .sum()
}

#[aoc(day14, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    // print(input);
    let mut grid = transpose(input.to_vec());

    let mut grid_cache: IndexMap<Vec<Vec<char>>, Vec<Vec<char>>> = IndexMap::new();
    let mut row_cache: HashMap<Vec<char>, Vec<char>> = HashMap::new();

    let mut cycle_start = None;
    let mut remaining = 1_000_000_000;
    while remaining > 0 {
        remaining -= 1;
        if let Some((i, _k, _next)) = grid_cache.get_full(&grid) {
            cycle_start = Some(i);
            break;
        } else {
            let mut result = grid.clone();
            for _ in 0..4 {
                tilt(&mut result, &mut row_cache);
                result = rotate(result);
            }
            grid_cache.insert(grid.clone(), result.clone());
            grid = result;
        }
    }
    let cycle_start = cycle_start.unwrap();
    let cycle_len = grid_cache.len() - cycle_start;
    let offset = remaining % cycle_len;
    // println!("cycle_start: {cycle_start}");
    // println!("cycle_len: {cycle_len}");
    // println!("remaining: {remaining}");
    // println!("offset: {offset}");
    let last = grid_cache.get_index(cycle_start + offset).unwrap().1.clone();
    grid = last;

    // println!("cycle: {}", grid_cache.len());
    // println!("after {cycle} cycles {}", "---".repeat(20));
    // grid = transpose(grid);
    // print(&grid);
    // grid = transpose(grid);

    let again = transpose(grid);
    let scores = (1..=again.len()).rev();
    again
        .into_iter()
        .zip(scores)
        .map(|(row, score)| row.iter().filter(|&c| *c == 'O').count() * score)
        .sum()
}

fn tilt(grid: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>) {
    for row in grid {
        if let Some(res) = cache.get(row) {
            *row = res.clone();
            continue;
        }
        let orig = row.clone();

        let positions = row.iter().positions(|&c| c == 'O').collect_vec();
        // println!("row {row:?}");
        for from in positions {
            if from == 0 {
                continue;
            }

            // println!("from {from:?}");
            if let Some(p) = row[0..from].iter().rposition(|&c| c == '#' || c == 'O') {
                let to = p + 1;
                // println!("to {to:?}");
                if from != to {
                    assert_eq!('.', row[to]);
                    row[to] = 'O';
                    row[from] = '.';
                }
            } else {
                // println!("zero");
                row[0] = 'O';
                row[from] = '.';
            }
        }
        cache.insert(orig, row.clone());
    }
}

fn rotate(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in &mut grid {
        row.reverse();
    }
    transpose(grid)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn print(rows: &[Vec<char>]) {
    print!("{:3} ", "");
    for i in 0..rows[0].len() {
        print!("{i:1}");
    }
    println!("");
    for (i, row) in rows.iter().enumerate() {
        println!("{i:3} {}", String::from_iter(row));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            )),
            136
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            )),
            64
        );
    }
}

