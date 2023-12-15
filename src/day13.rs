use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::new).collect()
}

struct Pattern {
    rows: Vec<Vec<char>>,
}

impl Pattern {
    fn new(input: &str) -> Self {
        let rows: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        assert!(rows.iter().all(|r| r.len() == rows[0].len()));

        Self {
            rows: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

#[aoc(day13, part1)]
fn part1(input: &[Pattern]) -> usize {
    let mut y_mirrors = Vec::new();
    let mut x_mirrors = Vec::new();
    for (_pid, pattern) in input.iter().enumerate() {
        if let (Some(y), _) = find_row(pattern) {
            y_mirrors.push(y);
            continue;
        }
        if let (Some(x), _) = find_col(pattern) {
            x_mirrors.push(x);
            continue;
        }
        panic!("no mirror found")
    }

    x_mirrors.iter().sum::<usize>() + (100 * y_mirrors.iter().sum::<usize>())
}

fn find_row(pattern: &Pattern) -> (Option<usize>, Option<usize>) {
    let pattern = pattern.rows.clone();
    let mut zero = None;
    let mut one = None;
    for y in 0..pattern.len() {
        let (above, below) = pattern.split_at(y);

        let len = above.len().min(below.len());
        if len == 0 {
            continue;
        }
        let mut below = below[0..len].to_vec();
        below.reverse();
        let dist = dist2(&above[(y - len)..y], &below[0..len]);
        if dist == 0 {
            // println!("y = {y}");
            zero = Some(y);
        } else if dist == 1 {
            one = Some(y);
        }
    }
    (zero, one)
}

fn find_col(pattern: &Pattern) -> (Option<usize>, Option<usize>) {
    let pattern = pattern.rows.clone();
    let mut zero = None;
    let mut one = None;
    for x in 1..(pattern[0].len()) {
        let dist = pattern
            .iter()
            .map(|row| {
                let (above, below) = row.split_at(x);
                let len = above.len().min(below.len());
                assert!(len > 0);
                let mut below = below[0..len].to_vec();
                below.reverse();
                dist(&above[(x - len)..x], &below[0..len])
            })
            .sum::<usize>();
        if dist == 0 {
            // println!("x = {x}");
            zero = Some(x);
        } else if dist == 1 {
            one = Some(x)
        }
    }
    (zero, one)
}

fn dist(a: &[char], b: &[char]) -> usize {
    a.iter().zip(b.iter()).filter(|(a, b)| a != b).count()
}

fn dist2(a: &[Vec<char>], b: &[Vec<char>]) -> usize {
    a.iter().zip(b.iter()).map(|(a, b)| dist(a, b)).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Pattern]) -> usize {
    let mut y_mirrors = Vec::new();
    let mut x_mirrors = Vec::new();
    for (_pid, pattern) in input.iter().enumerate() {
        if let (_, Some(y)) = find_row(pattern) {
            y_mirrors.push(y);
            continue;
        }
        if let (_, Some(x)) = find_col(pattern) {
            x_mirrors.push(x);
            continue;
        }
        panic!("no mirror found")
    }

    x_mirrors.iter().sum::<usize>() + (100 * y_mirrors.iter().sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            )),
            405
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            )),
            400
        );
    }
}
