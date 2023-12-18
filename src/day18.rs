use std::cell::Cell;

use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::IndexMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Instr {
    dir: Direction,
    len: usize,
    color: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| {
            let [d, l, c] = line
                .split_whitespace()
                .collect_vec()
                .try_into()
                .expect("three");
            let dir = match d {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("bad dir"),
            };
            Instr {
                dir,
                len: l.parse().expect("len"),
                color: c.to_string(),
            }
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Instr]) -> usize {
    let mut path = IndexMap::new();
    let current = (Cell::new(0), Cell::new(0));
    for instr in input {
        let (dir, diff) = match instr.dir {
            Direction::Right => (&current.0, 1),
            Direction::Left => (&current.0, -1),
            Direction::Up => (&current.1, -1),
            Direction::Down => (&current.1, 1),
        };
        if matches!(instr.dir, Direction::Up | Direction::Down) {
            path.insert((current.0.get(), current.1.get()), instr.dir);
        }
        for _ in 0..instr.len {
            dir.set(dir.get() + diff);
            path.insert((current.0.get(), current.1.get()), instr.dir);
        }
    }

    let min_x = path.keys().map(|p| p.0).min().unwrap();
    let max_x = path.keys().map(|p| p.0).max().unwrap();
    let min_y = path.keys().map(|p| p.1).min().unwrap();
    let max_y = path.keys().map(|p| p.1).max().unwrap();

    println!("({min_x}, {min_y}) to ({max_x}, {max_y})");
    let mut interior_count = 0;
    for y in min_y..=max_y {
        let mut entered = None;
        let mut vert_count = 0;
        for x in min_x..=max_x {
            if let Some(dir) = path.get(&(x, y)) {
                match dir {
                    Direction::Up | Direction::Down => {
                        print!("|");
                        if let Some(d) = entered {
                            if d != dir {
                                vert_count += 1;
                            }
                        } else {
                            entered = Some(dir);
                        }
                    }
                    Direction::Right | Direction::Left => {
                        print!("-");
                    }
                }
                interior_count += 1;
            } else {
                if entered.is_some() {
                    vert_count += 1;
                    entered = None;
                }
                if vert_count % 2 == 0 {
                    print!(".");
                } else {
                    print!("+");
                    interior_count += 1;
                }
            }
        }
        println!("");
    }
    interior_count
}

#[aoc(day18, part2)]
fn part2(input: &[Instr]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            )),
            62
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"
            )),
            8008
        );
    }
}

