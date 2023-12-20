use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use winnow::{
    ascii::alpha1,
    combinator::{alt, separated},
    PResult, Parser,
};

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast,
    Flipflop(bool),
    Conjunction(HashMap<String, bool>),
}

#[derive(Debug, Clone)]
struct Module {
    ty: ModuleType,
    name: String,
    destinations: Vec<String>,
}

impl Module {
    fn process(&mut self, from: &str, pulse: bool) -> Vec<(String, String, bool)> {
        match &mut self.ty {
            ModuleType::Broadcast => self
                .destinations
                .iter()
                .cloned()
                .map(|d| (d, self.name.clone(), pulse))
                .collect(),
            ModuleType::Flipflop(state) => {
                if pulse == true {
                    Vec::new()
                } else {
                    *state = !*state;
                    self.destinations
                        .iter()
                        .cloned()
                        .map(|d| (d, self.name.clone(), *state))
                        .collect()
                }
            }
            ModuleType::Conjunction(state) => {
                let entry = state.entry(from.into()).or_insert(false);
                *entry = pulse;
                let out = !state.values().all(|v| *v);
                self.destinations
                    .iter()
                    .cloned()
                    .map(|d| (d, self.name.clone(), out))
                    .collect()
            }
        }
    }
}

fn parse_module(input: &mut &str) -> PResult<Module> {
    let (name, ty) = alt((
        "broadcaster".map(|_| ("broadcast", ModuleType::Broadcast)),
        ('%', alpha1).map(|(_, name)| (name, ModuleType::Flipflop(false))),
        ('&', alpha1).map(|(_, name)| (name, ModuleType::Conjunction(Default::default()))),
    ))
    .parse_next(input)?;
    let name = String::from(name);

    let _ = " -> ".parse_next(input)?;

    let destinations = separated(1.., alpha1.map(String::from), ", ").parse_next(input)?;

    Ok(Module {
        ty,
        name,
        destinations,
    })
}

#[aoc_generator(day20)]
fn parse(input: &str) -> HashMap<String, Module> {
    input
        .lines()
        .map(|line| parse_module.parse(line).expect("module"))
        .map(|module| (module.name.clone(), module))
        .collect()
}

fn initialize(modules: &mut HashMap<String, Module>) {
    let conj_keys = modules
        .values()
        .filter(|m| matches!(m.ty, ModuleType::Conjunction(_)))
        .map(|m| m.name.clone())
        .collect_vec();
    let init_signals = modules
        .values()
        .flat_map(|m| {
            m.destinations
                .iter()
                .filter(|d| conj_keys.contains(d))
                .map(|d| (d.clone(), m.name.clone()))
        })
        .collect_vec();
    for (k, f) in init_signals {
        modules
            .get_mut(&k)
            .expect("init")
            .process(f.as_str(), false);
    }
}

#[aoc(day20, part1)]
fn part1(input: &HashMap<String, Module>) -> usize {
    let mut modules = input.clone();

    initialize(&mut modules);
    let mut high = 0;
    let mut low = 0;

    for _ in 0..1_000 {
        let mut signals = vec![(String::from("broadcast"), String::from("button"), false)];
        let mut next = Vec::new();
        while !signals.is_empty() {
            for (k, f, v) in signals.drain(..) {
                // println!("{f} -> {v} -> {k}");
                if v {
                    high += 1;
                } else {
                    low += 1;
                }
                if let Some(m) = modules.get_mut(&k) {
                    next.extend(m.process(&f, v));
                } else {
                    // println!("output: {k}, {f}, {v:?}");
                }
            }
            std::mem::swap(&mut signals, &mut next)
        }
    }
    high * low
}

#[aoc(day20, part2)]
fn part2(input: &HashMap<String, Module>) -> usize {
    let mut modules = input.clone();
    let inputs = ["vd", "ns", "bh", "dl"];
    let mut periods = HashMap::<String, usize>::new();
    let mut watching = HashSet::new();
    watching.extend(inputs);

    initialize(&mut modules);

    for n in 1.. {
        let mut signals = vec![(String::from("broadcast"), String::from("button"), false)];
        let mut next = Vec::new();
        while !signals.is_empty() {
            for (k, f, v) in signals.drain(..) {
                // println!("{f} -> {v} -> {k}");
                if watching.contains(f.as_str()) && k == "zh" && v {
                    if let Some(last) = periods.get(f.as_str()) {
                        periods.insert(f.clone(), last - n);
                        watching.remove(f.as_str());
                    } else {
                        periods.insert(f.clone(), n);
                    }
                }
                if let Some(m) = modules.get_mut(&k) {
                    next.extend(m.process(&f, v));
                } else if k == "rx" && !v {
                    return n;
                }
            }
            std::mem::swap(&mut signals, &mut next)
        }
        if watching.is_empty() {
            break;
        }
    }

    periods.values().cloned().reduce(num::integer::lcm).expect("lcm")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            part1(&parse(
                "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
            )),
            32000000
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            part1(&parse(
                "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
            )),
            11687500
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 8008);
    }
}

