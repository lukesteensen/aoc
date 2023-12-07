use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type GridNums = (HashMap<(i32, i32), char>, Vec<(u32, Vec<(i32, i32)>)>);

#[aoc_generator(day3)]
fn generator(input: &str) -> GridNums {
    let mut grid = HashMap::new();
    let mut numbers = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut current_number = None;
        let mut current_number_coords = Vec::new();
        for (col, char) in line.char_indices() {
            grid.insert((col as i32, row as i32), char);
            if let Some(digit) = char.to_digit(10) {
                if let Some(num) = current_number {
                    current_number = Some((num * 10) + digit)
                } else {
                    current_number = Some(digit)
                }
                current_number_coords.push((col as i32, row as i32));
            } else if let Some(num) = current_number.take() {
                numbers.push((num, std::mem::take(&mut current_number_coords)));
            }
        }
        if let Some(num) = current_number.take() {
            numbers.push((num, std::mem::take(&mut current_number_coords)));
        }
    }

    (grid, numbers)
}

#[aoc(day3, part1)]
fn part1((grid, numbers): &GridNums) -> u32 {
    numbers
        .iter()
        .filter_map(|(num, coords)| {
            let touches_symbol = coords
                .iter()
                .flat_map(|&(x, y)| {
                    [
                        (x, y + 1),
                        (x + 1, y + 1),
                        (x + 1, y),
                        (x + 1, y - 1),
                        (x, y - 1),
                        (x - 1, y - 1),
                        (x - 1, y),
                        (x - 1, y + 1),
                    ]
                })
                .filter(|coord| grid.contains_key(coord))
                .filter(|coord| !coords.contains(coord))
                .any(|coord| !grid[&coord].is_ascii_digit() && (grid[&coord] != '.'));

            if touches_symbol {
                Some(num)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2((grid, numbers): &GridNums) -> u32 {
    let mut gears: HashMap<(i32, i32), Vec<(u32, Vec<_>)>> = HashMap::new();
    for (num, coords) in numbers {
        let neighbors = coords
            .iter()
            .flat_map(|&(x, y)| {
                [
                    (x, y + 1),
                    (x + 1, y + 1),
                    (x + 1, y),
                    (x + 1, y - 1),
                    (x, y - 1),
                    (x - 1, y - 1),
                    (x - 1, y),
                    (x - 1, y + 1),
                ]
            })
            .filter(|coord| grid.contains_key(coord))
            .filter(|coord| !coords.contains(coord));
        for coord in neighbors {
            if grid[&coord] == '*' {
                let entry = gears.entry(coord).or_default();
                entry.push((*num, coords.clone()));
            }
        }
    }

    for (_coord, nums) in gears.iter_mut() {
        nums.sort();
        nums.dedup();
    }

    gears
        .values_mut()
        .map(|nums| nums.iter().map(|t| t.0).collect::<Vec<_>>())
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum()
}
