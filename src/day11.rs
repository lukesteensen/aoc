use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Input {
    empty_rows: Vec<isize>,
    empty_cols: Vec<isize>,
    galaxies: Vec<(isize, isize)>,
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Input {
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let empty_rows = rows
        .iter()
        .enumerate()
        .filter_map(|(i, row)| row.iter().all(|c| *c == '.').then_some(i as isize))
        .collect::<Vec<_>>();

    let empty_cols = (0..rows[0].len())
        .filter_map(|i| rows.iter().all(|row| row[i] == '.').then_some(i as isize))
        .collect::<Vec<_>>();

    let galaxies: Vec<_> = rows
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, c)| (*c == '#').then_some((x as isize, y as isize)))
        })
        .collect();

    Input {
        empty_rows,
        empty_cols,
        galaxies,
    }
}

fn translate(input: &Input, factor: isize) -> Vec<(isize, isize)> {
    input
        .galaxies
        .iter()
        .cloned()
        .map(|(x, y)| {
            let mut xx = x;
            let mut yy = y;
            for row in input.empty_rows.iter() {
                if y > *row {
                    yy += factor
                }
            }
            for col in input.empty_cols.iter() {
                if x > *col {
                    xx += factor
                }
            }
            (xx, yy)
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &Input) -> isize {
    let galaxies = translate(input, 1);

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((x, y), (xx, yy))| (xx - x).abs() + (yy - y).abs())
        .sum()
}

#[aoc(day11, part2)]
fn part2(input: &Input) -> isize {
    let galaxies = translate(input, 999_999);

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((x, y), (xx, yy))| (xx - x).abs() + (yy - y).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            )),
            374,
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 42);
    }
}
