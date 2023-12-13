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

    fn print(&self) {
        print!("{:3} ", "");
        for i in 0..self.rows[0].len() {
            print!("{i:1}");
        }
        println!("");
        for (i, row) in self.rows.iter().enumerate() {
            println!("{i:3} {}", String::from_iter(row));
        }
    }
}

#[aoc(day13, part1)]
fn part1(input: &[Pattern]) -> usize {
    let mut y_mirrors = Vec::new();
    let mut x_mirrors = Vec::new();
    for (pid, pattern) in input.iter().enumerate() {
        if let Some(y) = find_row(pattern) {
            y_mirrors.push(y);
            continue;
        }
        if let Some(x) = find_col(pattern) {
            x_mirrors.push(x);
            continue;
        }

        println!("pattern {pid}");
        pattern.print();
        let pattern = pattern.rows.clone();
        for y in 0..pattern.len() {
            let (above, below) = pattern.split_at(y);

            let len = above.len().min(below.len());
            if len == 0 {
                continue;
            }
            let mut below = below[0..len].to_vec();
            below.reverse();

            let aa = &above[(y - len)..y];
            let bb = &below[0..len];
            if y == 2 {
                println!("");
                for r in aa {
                    println!("{r:?}");
                }
                println!("{}", "--".repeat(10));
                for r in bb {
                    println!("{r:?}");
                }
            }
            if aa == bb {
                println!("y = {y}");
                break;
            }
        }
        panic!("no mirror found")
    }

    x_mirrors.iter().sum::<usize>() + (100 * y_mirrors.iter().sum::<usize>())
}

fn find_row(pattern: &Pattern) -> Option<usize> {
    let pattern = pattern.rows.clone();
    for y in 0..pattern.len() {
        let (above, below) = pattern.split_at(y);

        let len = above.len().min(below.len());
        if len == 0 {
            continue;
        }
        let mut below = below[0..len].to_vec();
        below.reverse();
        if &above[(y - len)..y] == &below[0..len] {
            // println!("y = {y}");
            return Some(y);
        }
    }
    None
}

fn find_col(pattern: &Pattern) -> Option<usize> {
    let pattern = pattern.rows.clone();
    for x in 0..pattern[0].len() {
        if pattern.iter().all(|row| {
            let (above, below) = row.split_at(x);
            let len = above.len().min(below.len());
            if len != 0 {
                let mut below = below[0..len].to_vec();
                below.reverse();
                &above[(x - len)..x] == &below[0..len]
            } else {
                false
            }
        }) {
            // println!("x = {x}");
            return Some(x);
        }
    }
    None
}

#[aoc(day13, part2)]
fn part2(input: &[Pattern]) -> usize {
    todo!()
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
        assert_eq!(part2(&parse("<EXAMPLE>")), 69);
    }
}

