use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::IndexMap;

#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("parse") as usize)
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
    dir_len: u8,
}

impl State {
    fn next(&self, grid: &[Vec<usize>]) -> Vec<Self> {
        let dirs = match self.dir {
            Direction::Right => {
                if self.dir_len < 3 {
                    vec![Direction::Right, Direction::Up, Direction::Down]
                } else {
                    vec![Direction::Up, Direction::Down]
                }
            }
            Direction::Left => {
                if self.dir_len < 3 {
                    vec![Direction::Left, Direction::Up, Direction::Down]
                } else {
                    vec![Direction::Up, Direction::Down]
                }
            }
            Direction::Up => {
                if self.dir_len < 3 {
                    vec![Direction::Up, Direction::Right, Direction::Left]
                } else {
                    vec![Direction::Right, Direction::Left]
                }
            }
            Direction::Down => {
                if self.dir_len < 3 {
                    vec![Direction::Down, Direction::Right, Direction::Left]
                } else {
                    vec![Direction::Right, Direction::Left]
                }
            }
        };

        dirs.into_iter()
            .map(|dir| {
                let dir_len = if dir == self.dir { self.dir_len + 1 } else { 1 };
                match dir {
                    Direction::Right => (self.x + 1 < grid[0].len()).then(|| {
                        let x = self.x + 1;
                        let y = self.y;
                        State { x, y, dir, dir_len }
                    }),
                    Direction::Left => (self.x > 0).then(|| {
                        let x = self.x - 1;
                        let y = self.y;
                        State { x, y, dir, dir_len }
                    }),
                    Direction::Up => (self.y > 0).then(|| {
                        let x = self.x;
                        let y = self.y - 1;
                        State { x, y, dir, dir_len }
                    }),
                    Direction::Down => (self.y + 1 < grid.len()).then(|| {
                        let x = self.x;
                        let y = self.y + 1;
                        State { x, y, dir, dir_len }
                    }),
                }
            })
            .flatten()
            .collect()
    }
}

#[aoc(day17, part1)]
fn part1(grid: &[Vec<usize>]) -> usize {
    let goal = (grid[0].len() - 1, grid.len() - 1);
    let start = State {
        x: 0,
        y: 0,
        dir: Direction::Right, // doesn't matter
        dir_len: 0,
    };
    let mut stack = IndexMap::new();
    stack.insert(start, 0);
    let mut visited = HashSet::new();
    while let Some((current, cost)) = stack.pop() {
        // println!("checking ({}, {}) with cost {}", current.x, current.y, cost);
        if (current.x, current.y) == goal {
            return cost;
        }
        visited.insert(current);
        for n in current.next(grid) {
            let c = grid[n.y][n.x];
            if !visited.contains(&n) && !stack.contains_key(&n) {
                // println!("  adding ({}, {}) with added cost {}", n.x, n.y, c);
                stack.insert(n, cost + c);
            } else if let Some(v) = stack.get_mut(&n) {
                if *v > cost + c {
                    // println!("  updating ({}, {}) to cum cost {}", n.x, n.y, cost + c);
                    *v = cost + c;
                }
            }
        }
        stack.sort_by_cached_key(|_k, v| *v);
        stack.reverse();
    }
    panic!("didn't find one");
}

#[aoc(day17, part2)]
fn part2(input: &[Vec<usize>]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            )),
            102
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 8008);
    }
}

