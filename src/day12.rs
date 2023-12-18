use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

struct Row {
    conditions: Vec<char>,
    groups: Vec<usize>,
}

impl Row {
    fn unfold(&self) -> Self {
        Self {
            conditions: itertools::intersperse(
                itertools::repeat_n(self.conditions.clone(), 5),
                vec!['?'],
            )
            .flatten()
            .collect(),
            groups: itertools::repeat_n(self.groups.clone(), 5)
                .flatten()
                .collect(),
        }
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let (conds, groups) = line.split_once(' ').expect("one");
            let conditions = conds.chars().collect();
            let groups = groups
                .split(',')
                .map(|s| s.parse::<usize>().expect("two"))
                .collect();
            Row { conditions, groups }
        })
        .collect()
}

fn group(input: &[char]) -> Vec<(char, usize)> {
    let mut last = 'X';
    let mut count = 0;
    let mut gs = Vec::new();
    for i in input {
        if *i != last && count > 0 {
            gs.push((last, count));
            count = 0;
        }
        count += 1;
        last = *i;
    }
    gs.push((last, count));
    // gs.retain(|(c, _)| *c != '.');
    gs
}

#[aoc(day12, part1)]
fn part1(input: &[Row]) -> usize {
    let mut combos = 0;
    for row in input {
        let mut possible = vec![row.conditions.clone()];
        while possible.iter().any(|v| v.iter().any(|c| *c == '?')) {
            possible = possible
                .into_iter()
                .flat_map(|mut v| {
                    if let Some(i) = v.iter().position(|&c| c == '?') {
                        let mut x = v.clone();
                        v[i] = '.';
                        x[i] = '#';
                        vec![v, x]
                    } else {
                        vec![v]
                    }
                })
                .collect();
        }
        for cond in possible {
            let gs = group(&cond);
            let counts: Vec<usize> = gs
                .into_iter()
                .filter_map(|(c, n)| (c == '#').then_some(n))
                .collect();
            if counts == row.groups {
                combos += 1
            }
        }
    }
    combos
}

#[aoc(day12, part2)]
fn part2(input: &[Row]) -> num::BigUint {
    let mut combos = num::BigUint::from(0u8);
    for row in input {
        let row = row.unfold();
        combos += arrangements(&row);
    }
    combos
}

fn arrangements(row: &Row) -> num::BigUint {
    let mut cache = HashMap::new();
    aux(row.conditions.clone(), row.groups.clone(), 0, &mut cache)
}

type Cache = HashMap<(Vec<char>, Vec<usize>, usize), num::BigUint>;

fn aux(tokens: Vec<char>, counts: Vec<usize>, count: usize, cache: &mut Cache) -> num::BigUint {
    let entry = cache.get(&(tokens.clone(), counts.clone(), count)).cloned();
    if let Some(v) = entry {
        v.to_owned()
    } else {
        let result = match (&tokens[..], &counts[..], count) {
            ([], [], 0) => 1u8.into(),
            ([], [c], count) if *c == count => 1u8.into(),
            (['?', ts @ ..], cs, count) => {
                let mut dot = vec!['.'];
                let mut spr = vec!['#'];
                dot.extend(ts);
                spr.extend(ts);
                aux(dot, cs.to_vec(), count, cache) + aux(spr, cs.to_vec(), count, cache)
            }
            (['.', ts @ ..], cs, count) if count == 0 => {
                aux(ts.to_vec(), cs.to_vec(), count, cache)
            }
            (['.', ts @ ..], [c, cs @ ..], count) if count != 0 && count == *c => {
                aux(ts.to_vec(), cs.to_vec(), 0, cache)
            }
            (['#', ts @ ..], cs, count) => aux(ts.to_vec(), cs.to_vec(), count + 1, cache),
            _ => 0u8.into(),
        };
        cache.insert((tokens.clone(), counts.clone(), count), result.clone());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            )),
            21
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            )),
            525152usize.into()
        );
    }
}
