use std::collections::HashMap;

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
}

#[derive(Debug)]
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

#[derive(Debug)]
enum Op {
    Lt(usize),
    Gt(usize),
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
    todo!()
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
            19114
        );
    }
}

