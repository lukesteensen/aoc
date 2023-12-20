use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::IndexMap;
use itertools::Itertools;
use winnow::{
    ascii::{alpha1, digit1},
    combinator::alt,
    PResult, Parser,
};

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<String> {
    input
        .trim_matches('\n')
        .split(',')
        .map(String::from)
        .collect_vec()
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

#[aoc(day15, part1)]
fn part1(input: &[String]) -> u32 {
    input.iter().map(|s| hash(s)).sum()
}

#[aoc(day15, part2)]
fn part2(input: &[String]) -> usize {
    let mut boxes: [IndexMap<&str, usize>; 256] = std::array::from_fn(|_| IndexMap::new());

    for op in input {
        let instr = parse_instr(&mut op.as_str()).expect("boop");

        let idx = hash(instr.label) as usize;
        let boxx = &mut boxes[idx];

        match instr.op {
            Op::Remove => {
                boxx.shift_remove(instr.label);
            }
            Op::Add(len) => {
                boxx.insert(instr.label, len);
            }
        }
        if false {
            println!("after {op:?}:");
            dump(&boxes);
            println!();
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.values().enumerate().map(move |(j, f)| {
                // dbg!((i, j, f));
                (i + 1) * (j + 1) * f
            })
        })
        .sum()
}

fn parse_instr<'a>(s: &mut &'a str) -> PResult<Instr<'a>> {
    Ok(Instr {
        label: alpha1.parse_next(s)?,
        op: parse_op(s)?,
    })
}

fn parse_op(s: &mut &str) -> PResult<Op> {
    alt((
        ('-'.map(|_| Op::Remove)),
        ('=', digit1.parse_to()).map(|(_, f)| Op::Add(f)),
    ))
    .parse_next(s)
}

struct Instr<'a> {
    label: &'a str,
    op: Op,
}

enum Op {
    Remove,
    Add(usize),
}

fn dump(bs: &[IndexMap<&str, usize>]) {
    for (i, b) in bs.iter().enumerate() {
        if b.is_empty() {
            continue;
        }
        print!("Box {i}:");
        for (_j, (k, v)) in b.iter().enumerate() {
            print!(" [{k} {v}]")
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )),
            1320
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )),
            145
        );
    }
}
