use std::{collections::BTreeSet, fmt};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    Start,
    Vert,
    Horiz,
    NE,
    NW,
    SW,
    SE,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Point")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl Point {
    fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> Self {
        Self { x, y, max_x, max_y }
    }

    fn lookup<T: Copy>(&self, grid: &[Vec<Option<T>>]) -> Option<T> {
        grid[self.y][self.x]
    }

    fn north(&self) -> Option<Self> {
        (self.y > 0).then(|| Self {
            y: self.y - 1,
            ..self.clone()
        })
    }

    fn south(&self) -> Option<Self> {
        (self.y < self.max_y).then_some(Self {
            y: self.y + 1,
            ..self.clone()
        })
    }

    fn east(&self) -> Option<Self> {
        (self.x < self.max_x).then_some(Self {
            x: self.x + 1,
            ..self.clone()
        })
    }

    fn west(&self) -> Option<Self> {
        (self.x > 0).then(|| Self {
            x: self.x - 1,
            ..self.clone()
        })
    }

    fn all(self) -> impl Iterator<Item = Self> {
        (0..=self.max_y).flat_map(move |y| {
            (0..=self.max_x).map(move |x| Self::new(x, y, self.max_x, self.max_y))
        })
    }
}

struct Input {
    grid: Vec<Vec<Option<Pipe>>>,
    path: Vec<Point>,
    start: Point,
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Input {
    let mut start = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = Some((x, y));
                        Some(Pipe::Start)
                    }
                    '|' => Some(Pipe::Vert),
                    '-' => Some(Pipe::Horiz),
                    'L' => Some(Pipe::NE),
                    'J' => Some(Pipe::NW),
                    '7' => Some(Pipe::SW),
                    'F' => Some(Pipe::SE),
                    '.' => None,
                    _ => panic!("unknown char"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (x, y) = start.expect("no start");
    let start = Point::new(x, y, grid[0].len() - 1, grid.len() - 1);

    let mut first = None;

    if let Some(west) = start.west() {
        if [Some(Pipe::Horiz), Some(Pipe::SE), Some(Pipe::NE)].contains(&west.lookup(&grid)) {
            first = Some(west);
        }
    }

    if let Some(east) = start.east() {
        if [Some(Pipe::Horiz), Some(Pipe::SW), Some(Pipe::NW)].contains(&east.lookup(&grid)) {
            first = Some(east);
        }
    }

    if let Some(north) = start.north() {
        if [Some(Pipe::Vert), Some(Pipe::SW), Some(Pipe::SE)].contains(&north.lookup(&grid)) {
            first = Some(north);
        }
    }

    if let Some(south) = start.south() {
        if [Some(Pipe::Vert), Some(Pipe::NW), Some(Pipe::NE)].contains(&south.lookup(&grid)) {
            first = Some(south);
        }
    }

    let mut last = start.clone();
    let Some(mut current) = first else {
        panic!("no path from start")
    };
    let mut path = vec![start.clone(), current.clone()];

    loop {
        let next = match current.lookup(&grid) {
            Some(Pipe::Start) => break,
            Some(Pipe::Vert) => [current.north(), current.south()],
            Some(Pipe::Horiz) => [current.east(), current.west()],
            Some(Pipe::NE) => [current.north(), current.east()],
            Some(Pipe::NW) => [current.north(), current.west()],
            Some(Pipe::SE) => [current.south(), current.east()],
            Some(Pipe::SW) => [current.south(), current.west()],
            None => panic!("no pipe"),
        };
        let next = next
            .into_iter()
            .flatten()
            .find(|p| p != &last)
            .expect("should be one");
        last = current;
        current = next;
        path.push(current.clone())
    }

    Input { grid, path, start }
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> usize {
    input.path.len() / 2
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> usize {
    let Input { path, grid, start } = input;

    let all = start.clone().all().collect::<BTreeSet<_>>();
    let path = path.clone().into_iter().collect::<BTreeSet<_>>();
    let mut unknown = &all - &path;

    let mut maybe_interior = BTreeSet::new();

    while let Some(node) = unknown.pop_first() {
        let mut found_edge = false;
        let mut seen = path.clone();
        let mut to_try = vec![node];

        while let Some(n) = to_try.pop() {
            seen.insert(n.clone());

            for n in [n.north(), n.south(), n.east(), n.west()] {
                if let Some(n) = n {
                    if !seen.contains(&n) {
                        to_try.push(n);
                    }
                } else {
                    found_edge = true;
                }
            }
        }

        unknown = &unknown - &seen;
        if !found_edge {
            maybe_interior.insert(&seen - &path);
        }
    }

    let expanded = grid
        .iter()
        .flat_map(|row| {
            let mut a = Vec::new();
            let mut b = Vec::new();
            let mut c = Vec::new();
            for x in row {
                match x {
                    None => {
                        a.extend([None, None, None]);
                        b.extend([None, None, None]);
                        c.extend([None, None, None]);
                    }
                    Some(Pipe::Start) => {
                        a.extend([None, Some(()), None]);
                        b.extend([Some(()), Some(()), Some(())]);
                        c.extend([None, Some(()), None]);
                    }
                    Some(Pipe::Horiz) => {
                        a.extend([None, None, None]);
                        b.extend([Some(()), Some(()), Some(())]);
                        c.extend([None, None, None]);
                    }
                    Some(Pipe::Vert) => {
                        a.extend([None, Some(()), None]);
                        b.extend([None, Some(()), None]);
                        c.extend([None, Some(()), None]);
                    }
                    Some(Pipe::NE) => {
                        a.extend([None, Some(()), None]);
                        b.extend([None, Some(()), Some(())]);
                        c.extend([None, None, None]);
                    }
                    Some(Pipe::NW) => {
                        a.extend([None, Some(()), None]);
                        b.extend([Some(()), Some(()), None]);
                        c.extend([None, None, None]);
                    }
                    Some(Pipe::SE) => {
                        a.extend([None, None, None]);
                        b.extend([None, Some(()), Some(())]);
                        c.extend([None, Some(()), None]);
                    }
                    Some(Pipe::SW) => {
                        a.extend([None, None, None]);
                        b.extend([Some(()), Some(()), None]);
                        c.extend([None, Some(()), None]);
                    }
                }
            }
            [a, b, c]
        })
        .collect::<Vec<_>>();

    let mut interior = 0;
    for mut region in maybe_interior {
        let count = region.len();
        let mut found_edge = false;
        let mut seen = BTreeSet::new();
        let mut start = region.pop_first().expect("region first");
        start.x *= 3;
        start.y *= 3;
        start.max_x *= 3;
        start.max_y *= 3;
        let mut to_try = vec![start];

        while let Some(n) = to_try.pop() {
            seen.insert(n.clone());

            for n in [n.north(), n.south(), n.east(), n.west()] {
                if let Some(n) = n {
                    if !seen.contains(&n) && n.lookup(&expanded).is_none() {
                        to_try.push(n);
                    }
                } else {
                    found_edge = true;
                    break;
                }
            }
        }

        if !found_edge {
            interior += count;
        }
    }

    interior
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                ".....
.S-7.
.|.|.
.L-J.
....."
            )),
            4
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            part1(&parse(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            )),
            8
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            )),
            4
        );
    }
}
