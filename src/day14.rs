use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    // print(input);
    let mut flipped = transpose(input.to_vec());
    // println!("{}", "---".repeat(10));
    // print(&flipped);
    // println!("{}", "---".repeat(10));

    tilt(&mut flipped);

    let again = transpose(flipped);
    // println!("{}", "---".repeat(10));
    // print(&again);
    let scores = (1..=again.len()).rev();
    again
        .into_iter()
        .zip(scores)
        .map(|(row, score)| row.iter().filter(|&c| *c == 'O').count() * score)
        .sum()
}

#[aoc(day14, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    let mut flipped = transpose(input.to_vec());

    for _cycle in 0..1000000000 {
        for _ in 0..4 {
            tilt(&mut flipped);
            flipped = rotate(flipped);
        }
    }

    let again = transpose(flipped);
    let scores = (1..=again.len()).rev();
    again
        .into_iter()
        .zip(scores)
        .map(|(row, score)| row.iter().filter(|&c| *c == 'O').count() * score)
        .sum()
}

fn tilt(grid: &mut Vec<Vec<char>>) {
    for row in grid {
        let mut moved = true;
        while moved == true {
            moved = false;
            for i in 1..row.len() {
                let x = i - 1;
                let y = i;
                if row[x] == '.' && row[y] == 'O' {
                    row[x] = 'O';
                    row[y] = '.';
                    moved = true;
                }
            }
        }
    }
}

fn rotate(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in &mut grid {
        row.reverse();
    }
    transpose(grid)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn print(rows: &[Vec<char>]) {
    print!("{:3} ", "");
    for i in 0..rows[0].len() {
        print!("{i:1}");
    }
    println!("");
    for (i, row) in rows.iter().enumerate() {
        println!("{i:3} {}", String::from_iter(row));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            )),
            136
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 69);
    }
}

