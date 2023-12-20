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
    solve(input)
}

fn solve(input: &[Instr]) -> usize {
    let mut path = IndexMap::new();
    let current = (Cell::new(0isize), Cell::new(0isize));
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

    let path_len = path.len();

    let compact = path.into_iter().into_group_map_by(|((_x, y), _dir)| *y);
    let mut interior_count = 0;
    for (_y, mut row) in compact {
        row.sort_unstable_by_key(|((x, _y), _dir)| *x);
        let mut last_x = None;
        let mut entered = None;
        let mut vert_count = 0;
        for ((x, _y), dir) in row {
            let dx = last_x.map(|lx| x - lx).unwrap_or(0) as usize;
            if dx > 1 {
                if let Some(_d) = entered.take() {
                    vert_count += 1;
                }

                let gap = dx - 1;
                if vert_count % 2 == 1 {
                    interior_count += gap;
                }

                entered = Some(dir);
            } else {
                match dir {
                    Direction::Up | Direction::Down => {
                        if let Some(d) = entered.take() {
                            if d == dir {
                                vert_count += 1;
                            }
                        } else {
                            entered = Some(dir);
                        }
                    }
                    Direction::Right | Direction::Left => {}
                }
            }
            last_x = Some(x);
        }
    }

    interior_count + path_len
}

#[aoc(day18, part2)]
fn part2(input: &[Instr]) -> usize {
    let expanded = input
        .iter()
        .map(|instr| {
            let len = instr.color.chars().skip(2).take(5).collect::<String>();
            let dir = instr.color.chars().skip(7).take(1).collect::<String>();

            let len = usize::from_str_radix(&len, 16).expect("parse color");
            let dir = match dir.as_str() {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("bad dir"),
            };

            Instr {
                dir,
                len,
                color: String::new(),
            }
        })
        .collect_vec();
    solve(&expanded)
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
            952408144115
        );
    }
}
