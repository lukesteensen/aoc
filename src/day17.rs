use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

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
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type DirFn = fn(&State) -> Vec<Direction>;

impl State {
    fn next(&self, grid: &[Vec<usize>], dir_fn: DirFn) -> Vec<Self> {
        let dirs = dir_fn(self);

        dirs.into_iter()
            .filter_map(|dir| {
                let dir_len = if dir == self.dir { self.dir_len + 1 } else { 1 };
                match dir {
                    Direction::Right => (self.x + 1 < grid[0].len()).then(|| {
                        let x = self.x + 1;
                        let y = self.y;
                        State {
                            x,
                            y,
                            dir,
                            dir_len,
                            cost: self.cost + grid[y][x],
                        }
                    }),
                    Direction::Left => (self.x > 0).then(|| {
                        let x = self.x - 1;
                        let y = self.y;
                        State {
                            x,
                            y,
                            dir,
                            dir_len,
                            cost: self.cost + grid[y][x],
                        }
                    }),
                    Direction::Up => (self.y > 0).then(|| {
                        let x = self.x;
                        let y = self.y - 1;
                        State {
                            x,
                            y,
                            dir,
                            dir_len,
                            cost: self.cost + grid[y][x],
                        }
                    }),
                    Direction::Down => (self.y + 1 < grid.len()).then(|| {
                        let x = self.x;
                        let y = self.y + 1;
                        State {
                            x,
                            y,
                            dir,
                            dir_len,
                            cost: self.cost + grid[y][x],
                        }
                    }),
                }
            })
            .collect()
    }

    fn costless(&self) -> Self {
        Self { cost: 0, ..*self }
    }
}

#[aoc(day17, part1)]
fn part1(grid: &[Vec<usize>]) -> usize {
    solve(grid, normal)
}

#[aoc(day17, part2)]
fn part2(grid: &[Vec<usize>]) -> usize {
    solve(grid, ultra)
}

fn solve(grid: &[Vec<usize>], dir_fn: DirFn) -> usize {
    let goal = (grid[0].len() - 1, grid.len() - 1);
    let start = State {
        x: 0,
        y: 0,
        dir: Direction::Right, // doesn't matter
        dir_len: 0,
        cost: 0,
    };
    let mut heap = BinaryHeap::new();
    let mut heats = HashMap::new();
    let mut visited = HashSet::new();

    heap.push(start);
    while let Some(current) = heap.pop() {
        // println!("checking ({}, {}) with cost {}", current.x, current.y, cost);
        if (current.x, current.y) == goal {
            return current.cost;
        }

        if !visited.insert(current.costless()) {
            continue;
        }

        for n in current.next(grid, dir_fn) {
            if let Some(&existing) = heats.get(&n.costless()) {
                if existing < n.cost {
                    continue;
                }
            }
            heats.insert(n.costless(), n.cost);
            heap.push(n);
        }
    }
    panic!("didn't find one");
}

fn normal(state: &State) -> Vec<Direction> {
    match state.dir {
        Direction::Right => {
            if state.dir_len < 3 {
                vec![Direction::Right, Direction::Up, Direction::Down]
            } else {
                vec![Direction::Up, Direction::Down]
            }
        }
        Direction::Left => {
            if state.dir_len < 3 {
                vec![Direction::Left, Direction::Up, Direction::Down]
            } else {
                vec![Direction::Up, Direction::Down]
            }
        }
        Direction::Up => {
            if state.dir_len < 3 {
                vec![Direction::Up, Direction::Right, Direction::Left]
            } else {
                vec![Direction::Right, Direction::Left]
            }
        }
        Direction::Down => {
            if state.dir_len < 3 {
                vec![Direction::Down, Direction::Right, Direction::Left]
            } else {
                vec![Direction::Right, Direction::Left]
            }
        }
    }
}

fn ultra(state: &State) -> Vec<Direction> {
    match state.dir {
        Direction::Right => {
            if state.dir_len < 4 {
                vec![Direction::Right]
            } else if state.dir_len < 10 {
                vec![Direction::Right, Direction::Up, Direction::Down]
            } else {
                vec![Direction::Up, Direction::Down]
            }
        }
        Direction::Left => {
            if state.dir_len < 4 {
                vec![Direction::Left]
            } else if state.dir_len < 10 {
                vec![Direction::Left, Direction::Up, Direction::Down]
            } else {
                vec![Direction::Up, Direction::Down]
            }
        }
        Direction::Up => {
            if state.dir_len < 4 {
                vec![Direction::Up]
            } else if state.dir_len < 10 {
                vec![Direction::Up, Direction::Right, Direction::Left]
            } else {
                vec![Direction::Right, Direction::Left]
            }
        }
        Direction::Down => {
            if state.dir_len < 4 {
                vec![Direction::Down]
            } else if state.dir_len < 10 {
                vec![Direction::Down, Direction::Right, Direction::Left]
            } else {
                vec![Direction::Right, Direction::Left]
            }
        }
    }
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
        assert_eq!(
            part2(&parse(
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
            94
        );
    }
}
