use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

fn valid(input: &[char], groups: &[usize]) -> bool {
    let gs = group(input);
    let first_q = gs.iter().position(|(c, _)| *c == '?').unwrap_or(gs.len());
    if first_q > 3 {
        let counts: Vec<usize> = gs
            .into_iter()
            .take(first_q)
            .filter_map(|(c, n)| (c == '#').then_some(n))
            .collect();

        counts == &groups[0..counts.len().min(groups.len())]
    } else {
        gs.into_iter().filter(|(c, _)| *c == '#').count() <= groups.iter().sum()
    }
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

fn part2_bad(input: &[Row]) -> usize {
    let mut combos = 0;
    let rows = input.len();
    for (n, row) in input.iter().enumerate() {
        println!("row {n} of {rows}");
        let row = row.unfold();
        let mut possible = vec![row.conditions.clone()];
        while possible.iter().any(|v| v.iter().any(|c| *c == '?')) {
            println!("  {} possible", possible.len());
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
                .filter(|p| valid(p, &row.groups))
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
    let cache = Rc::new(RefCell::new(HashMap::new()));
    aux(row.conditions.clone(), row.groups.clone(), 0, cache)
}

fn aux(
    tokens: Vec<char>,
    counts: Vec<usize>,
    count: usize,
    cache: Rc<RefCell<HashMap<(Vec<char>, Vec<usize>, usize), num::BigUint>>>,
) -> num::BigUint {
    let b = cache.borrow();
    let entry = b.get(&(tokens.clone(), counts.clone(), count)).cloned();
    drop(b);
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
                aux(dot, cs.to_vec(), count, Rc::clone(&cache))
                    + aux(spr, cs.to_vec(), count, Rc::clone(&cache))
            }
            (['.', ts @ ..], cs, count) if count == 0 => {
                aux(ts.to_vec(), cs.to_vec(), count, Rc::clone(&cache))
            }
            (['.', ts @ ..], [c, cs @ ..], count) if count != 0 && count == *c => {
                aux(ts.to_vec(), cs.to_vec(), 0, Rc::clone(&cache))
            }
            (['#', ts @ ..], cs, count) => {
                aux(ts.to_vec(), cs.to_vec(), count + 1, Rc::clone(&cache))
            }
            _ => 0u8.into(),
        };
        cache
            .borrow_mut()
            .insert((tokens.clone(), counts.clone(), count), result.clone());
        result
    }
}

fn solve(mut segment: Vec<char>, rem: &[char], goal: &[usize]) -> num::BigUint {
    if rem.is_empty() {
        if groups(&segment) == goal {
            // println!("good");
            return 1u8.into();
        } else {
            // println!("bad");
            return 0u8.into();
        }
    } else if !compatible(&segment, rem, goal) {
        // println!("incompat");
        return 0u8.into();
    }

    let next_q = rem.iter().position(|&c| c == '?').unwrap_or(rem.len());
    let (add, rem) = rem.split_at(next_q);
    if !rem.is_empty() {
        assert_eq!('?', rem[0]);
    }

    segment.extend(add);

    let mut one = segment.clone();
    let mut two = segment;
    one.push('.');
    two.push('#');
    if rem.len() > 1 {
        solve(one, &rem[1..], goal) + solve(two, &rem[1..], goal)
    } else {
        solve(one, &[], goal) + solve(two, &[], goal)
    }
}

fn compatible(segment: &[char], rem: &[char], goal: &[usize]) -> bool {
    if segment.is_empty() {
        return true;
    }

    let seg_count = segment.iter().filter(|&c| *c == '#').count();
    let rem_count = rem.iter().filter(|&c| *c == '#').count();
    let rem_qs = rem.iter().filter(|&c| *c == '?').count();
    let goal_tot = goal.iter().sum();

    if seg_count + rem_count > goal_tot {
        return false;
    }

    if seg_count + rem_count + rem_qs < goal_tot {
        return false;
    }

    let gs = groups(segment); // 1, 1
    if gs.is_empty() {
        return true;
    }
    let up_to = gs.len() - 1; // 1
    if up_to == 0 {
        return true;
    }
    goal.len() >= gs.len() && gs[0..up_to] == goal[0..up_to] && gs[up_to] <= goal[up_to]
}

fn groups(input: &[char]) -> Vec<usize> {
    input
        .split(|&c| c == '.')
        .filter_map(|s| (!s.is_empty()).then_some(s.len()))
        .collect()
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
