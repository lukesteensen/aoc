use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().expect("parse reading"))
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i64>]) -> i64 {
    let mut next_values = Vec::new();
    for line in input {
        let mut rows = vec![line.clone()];
        while !rows.last().expect("last").iter().all(|n| *n == 0) {
            rows.push(
                rows.last()
                    .expect("last")
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect(),
            )
        }
        let mut last = 0;
        for row in rows.iter_mut().rev() {
            last += row.last().unwrap();
            row.push(last);
        }
        next_values.push(*rows[0].last().unwrap());
    }
    next_values.iter().sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i64>]) -> i64 {
    let mut next_values = Vec::new();
    for line in input {
        let mut rows = vec![line.clone()];
        rows[0].reverse();
        while !rows.last().expect("last").iter().all(|n| *n == 0) {
            rows.push(
                rows.last()
                    .expect("last")
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect(),
            )
        }
        let mut last = 0;
        for row in rows.iter_mut().rev() {
            last += row.last().unwrap();
            row.push(last);
        }
        next_values.push(*rows[0].last().unwrap());
    }
    next_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            )),
            114
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 42);
    }
}
