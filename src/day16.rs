use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::IndexSet;

#[aoc_generator(day16)]
fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

#[derive(Debug)]
enum Tile {
    Empty,
    MirrorUp,
    MirrorDown,
    SplitterVert,
    SplitterHoriz,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::MirrorUp,
            '\\' => Tile::MirrorDown,
            '|' => Tile::SplitterVert,
            '-' => Tile::SplitterHoriz,
            _ => panic!("unknown tile"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Beam {
    fn next(&self, grid: &[Vec<Tile>]) -> Vec<Self> {
        // println!("current: {self:?}");
        let dirs = match &grid[self.y][self.x] {
            Tile::Empty => {
                vec![self.dir]
            }
            Tile::MirrorUp => {
                vec![match self.dir {
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                }]
            }
            Tile::MirrorDown => {
                vec![match self.dir {
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                }]
            }
            Tile::SplitterVert => match self.dir {
                Direction::Right | Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Up | Direction::Down => vec![self.dir],
            },
            Tile::SplitterHoriz => match self.dir {
                Direction::Right | Direction::Left => vec![self.dir],
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            },
        };

        dirs.into_iter()
            .filter_map(|dir| match dir {
                Direction::Right => (self.x + 1 < grid[0].len()).then_some(Beam {
                    x: self.x + 1,
                    y: self.y,
                    dir,
                }),
                Direction::Left => (self.x > 0).then_some(Beam {
                    x: self.x - 1,
                    y: self.y,
                    dir,
                }),
                Direction::Up => (self.y > 0).then_some(Beam {
                    x: self.x,
                    y: self.y - 1,
                    dir,
                }),
                Direction::Down => (self.y + 1 < grid.len()).then_some(Beam {
                    x: self.x,
                    y: self.y + 1,
                    dir,
                }),
            })
            .collect()
        // println!("new coords: ({x}, {y})");
    }
}

#[aoc(day16, part1)]
fn part1(grid: &[Vec<Tile>]) -> usize {
    solve(grid, 0, 0, Direction::Right)
}

fn solve(grid: &[Vec<Tile>], x: usize, y: usize, dir: Direction) -> usize {
    let mut beams = IndexSet::new();
    beams.insert(Beam { x, y, dir });
    let mut energized = HashSet::new();
    let mut prev_states = HashSet::new();
    while let Some(beam) = beams.pop() {
        energized.insert((beam.x, beam.y));
        if prev_states.insert(beam) {
            beams.extend(beam.next(grid))
        }
    }
    // println!("{energized:?}");
    energized.len()
}

#[aoc(day16, part2)]
fn part2(grid: &[Vec<Tile>]) -> usize {
    let left = (0..grid.len()).map(|y| solve(grid, 0, y, Direction::Right));
    let right = (0..grid.len()).map(|y| solve(grid, grid[0].len() - 1, y, Direction::Left));
    let top = (0..grid[0].len()).map(|x| solve(grid, x, 0, Direction::Down));
    let bottom = (0..grid[0].len()).map(|x| solve(grid, x, grid.len() - 1, Direction::Up));

    left.chain(right).chain(top).chain(bottom).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            )),
            46,
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            )),
            51
        );
    }
}
