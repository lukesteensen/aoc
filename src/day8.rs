use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;

struct Input {
    directions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let directions = lines.next().expect("instr").chars().collect();
    let _ = lines.next().expect("blank");
    let nodes = lines
        .map(|line| {
            let (id, dirs) = line.split_once(" = ").expect("once");
            let (left, right) = dirs
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .expect("dirs");
            (id.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();
    Input { directions, nodes }
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> u32 {
    let Input { directions, nodes } = input;

    let mut directions = std::iter::repeat(directions).flatten();

    let mut id = "AAA";
    let mut steps = 0;
    loop {
        steps += 1;
        let next_dir = directions.next().unwrap();
        id = match next_dir {
            'L' => &nodes[id].0,
            'R' => &nodes[id].1,
            _ => panic!("bad dir"),
        };
        if id == "ZZZ" {
            break;
        }
    }
    steps
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    let Input { directions, nodes } = input;

    // let dir_count = directions.len();
    // println!("directions: {dir_count}");

    let positions: Vec<&str> = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(String::as_str)
        .collect();
    let mut paths = Vec::new();
    for mut pos in positions {
        let mut path = Vec::new();
        let mut directions = std::iter::repeat(directions.iter().enumerate()).flatten();
        loop {
            let (dir_idx, next_dir) = directions.next().unwrap();
            if path.contains(&(pos, dir_idx)) {
                path.push((pos, dir_idx));
                break;
            }
            path.push((pos, dir_idx));
            pos = match next_dir {
                'L' => &nodes[pos].0,
                'R' => &nodes[pos].1,
                _ => panic!("bad dir"),
            };
        }
        paths.push(path);
    }

    let mut lens = Vec::new();
    for mut path in paths {
        let last = path.pop().unwrap();
        let loop_start = path.iter().position(|p| p == &last).unwrap();
        let loop_len = path.len() - loop_start;
        let zs = path
            .iter()
            .enumerate()
            .filter(|(_, (p, _))| p.ends_with('Z'))
            .collect::<Vec<_>>();

        // println!("total len: {}", path.len());
        // println!("zs: {zs:?}");
        // println!("loop start: {loop_start}");
        // println!("loop len: {}", loop_len);
        // println!("loop len / dir_count: {}", loop_len / dir_count);
        // println!("loop len % dir_count: {}\n", loop_len % dir_count);

        // these seem important but fail for the example??
        assert_eq!(1, zs.len());
        let end_offset = zs[0].0;
        assert_eq!(end_offset, loop_len);
        lens.push(path.len() - loop_start);
    }
    lens.into_iter().reduce(|a, x| a.lcm(&x)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            )),
            2
        );
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(
            part1(&parse(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            )),
            6
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            )),
            6
        );
    }
}

