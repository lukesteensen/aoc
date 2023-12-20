use core::fmt;
use std::{collections::HashMap, ops::Range};

use aoc_runner_derive::{aoc, aoc_generator};
use winnow::{
    ascii::{alpha1, digit1},
    combinator::{delimited, opt, separated},
    token::one_of,
    PResult, Parser,
};

struct Input {
    parts: Vec<Part>,
    workflows: HashMap<String, Workflow>,
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRange {
    fn combos(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }

    fn get_mut(&mut self, field: Field) -> &mut Range<usize> {
        match field {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }
    }
}

impl Default for PartRange {
    fn default() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    cond: Option<Cond>,
    dest: String,
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<&str> {
        if let Some(cond) = &self.cond {
            cond.apply(part).then_some(&self.dest)
        } else {
            Some(&self.dest)
        }
    }
}

#[derive(Debug)]
struct Cond {
    field: Field,
    op: Op,
}

impl Cond {
    fn apply(&self, part: &Part) -> bool {
        let x = match self.field {
            Field::X => &part.x,
            Field::M => &part.m,
            Field::A => &part.a,
            Field::S => &part.s,
        };
        match &self.op {
            Op::Lt(y) => x < y,
            Op::Gt(y) => x > y,
        }
    }

    fn split(&self, part_range: PartRange) -> (PartRange, PartRange) {
        let mut pass = part_range.clone();
        let mut fail = part_range;

        let pass_range = pass.get_mut(self.field);
        let fail_range = fail.get_mut(self.field);
        match &self.op {
            Op::Lt(y) => {
                if pass_range.end > *y {
                    pass_range.end = *y;
                }
                if fail_range.start < *y {
                    fail_range.start = *y;
                }
                assert!(pass_range.clone().all(|x| x < *y));
                assert!(!fail_range.clone().any(|x| x < *y));
            }
            Op::Gt(y) => {
                if pass_range.start < y + 1 {
                    pass_range.start = y + 1;
                }
                if fail_range.end > *y + 1 {
                    fail_range.end = *y + 1;
                }
                assert!(pass_range.clone().all(|x| x > *y));
                assert!(!fail_range.clone().any(|x| x > *y));
            }
        }

        (pass, fail)
    }
}

#[derive(Debug, Clone, Copy)]
enum Field {
    X,
    M,
    A,
    S,
}

impl Cond {
    fn new((field, op, value, _): (&str, char, &str, char)) -> Self {
        let field = match field {
            "x" => Field::X,
            "m" => Field::M,
            "a" => Field::A,
            "s" => Field::S,
            _ => panic!("bad field"),
        };
        let value: usize = value.parse().expect("val");
        let op = match op {
            '<' => Op::Lt(value),
            '>' => Op::Gt(value),
            _ => panic!("bad op"),
        };
        Self { field, op }
    }
}

impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?} {}", self.field, self.op))
    }
}

#[derive(Debug)]
enum Op {
    Lt(usize),
    Gt(usize),
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Lt(v) => f.write_fmt(format_args!("< {}", v)),
            Op::Gt(v) => f.write_fmt(format_args!("> {}", v)),
        }
    }
}

fn parse_workflow(input: &mut &str) -> PResult<Workflow> {
    let (name, rules) = (
        alpha1.map(String::from),
        delimited('{', separated(1.., parse_rule, ','), '}'),
    )
        .parse_next(input)?;
    Ok(Workflow { name, rules })
}

fn parse_rule(input: &mut &str) -> PResult<Rule> {
    let (cond, dest) = (
        opt((alpha1, one_of(['<', '>']), digit1, ':').map(Cond::new)),
        alpha1.map(String::from),
    )
        .parse_next(input)?;

    Ok(Rule { cond, dest })
}

fn parse_part(input: &mut &str) -> PResult<Part> {
    let _ = "{x=".parse_next(input)?;
    let x = digit1.parse_to().parse_next(input)?;
    let _ = ",m=".parse_next(input)?;
    let m = digit1.parse_to().parse_next(input)?;
    let _ = ",a=".parse_next(input)?;
    let a = digit1.parse_to().parse_next(input)?;
    let _ = ",s=".parse_next(input)?;
    let s = digit1.parse_to().parse_next(input)?;
    let _ = "}".parse_next(input)?;
    Ok(Part { x, m, a, s })
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let (workflows, parts) = input.split_once("\n\n").expect("once");
    let workflows = workflows
        .lines()
        .map(|line| parse_workflow.parse(line).expect("workflow"))
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();

    let parts = parts
        .lines()
        .map(|line| parse_part.parse(line).expect("part"))
        .collect();

    Input { parts, workflows }
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let mut total = 0;
    for part in &input.parts {
        let mut key = "in";
        'part: loop {
            let workflow = &input.workflows[key];
            'rules: for rule in &workflow.rules {
                if let Some(dest) = rule.apply(part) {
                    match dest {
                        "A" => {
                            total += part.sum();
                            break 'part;
                        }
                        "R" => break 'part,
                        next => {
                            key = next;
                            break 'rules;
                        }
                    }
                }
            }
        }
    }
    total
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    fn rec<'a>(
        workflows: &'a HashMap<String, Workflow>,
        mut range: PartRange,
        key: &'a str,
    ) -> usize {
        if range.is_empty() {
            // println!("empty range = 0\n");
            return 0;
        }

        if key == "A" {
            // println!("accepted {range:?} = {}\n", range.combos());
            return range.combos();
        } else if key == "R" {
            // println!("rejected = 0\n");
            return 0;
        }

        // println!("starting range {range:?} key {key}");

        let mut total = 0;
        let w = &workflows[key];
        for rule in &w.rules {
            if range.is_empty() {
                return 0;
            }
            if let Some(cond) = &rule.cond {
                // println!("splitting with cond {cond}");
                let (pass, fail) = cond.split(range);
                total += rec(workflows, pass, &rule.dest);
                range = fail;
                // println!("continuing with fail range {range:?}");
            } else {
                total += rec(workflows, range, rule.dest.as_str());
                break; // this will be the last anyway
            }
        }
        total
    }

    rec(&input.workflows, PartRange::default(), "in")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            )),
            19114
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            )),
            167409079868000
        );
    }
}
