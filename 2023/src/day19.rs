use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};

use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
//maybe we need to return a vec of valid splits

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day19, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

type Part1Parsed = (Arc<HashMap<String, Vec<WorkFlow>>>, Arc<Vec<Vec<PartType>>>);
// #[aoc_generator(day19)]
fn parse(input: &str) -> Part1Parsed {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut lines = input.lines();
    workflows.insert("A".to_owned(), vec![WorkFlow::Accept]);
    workflows.insert("R".to_owned(), vec![WorkFlow::Reject]);
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut vals = line.split("{");
        let name = vals.next().unwrap();
        let wk = vals.next().unwrap().strip_suffix("}").unwrap();
        let parts = wk.split(",").collect::<Vec<_>>();
        let mut instructions = Vec::new();
        for p in parts {
            let pvals = p.split(":").collect::<Vec<_>>();
            if pvals.len() > 1 {
                let condition = pvals[0];
                let mut first_two = condition.chars().take(2);
                let amt = condition[2..].parse::<usize>().unwrap();
                let p_type = match first_two.next().unwrap() {
                    'x' => PartType::X(amt),
                    'm' => PartType::M(amt),
                    'a' => PartType::A(amt),
                    's' => PartType::S(amt),
                    _ => unreachable!(),
                };

                let cond = match first_two.next().unwrap() {
                    '<' => Cond::Lt(p_type),
                    '>' => Cond::Gt(p_type),
                    _ => unreachable!(),
                };

                let next_cmd = pvals[1].to_owned();
                instructions.push(WorkFlow::Condition(cond, next_cmd));
            } else {
                let next_cmd = pvals[0].to_owned();
                instructions.push(WorkFlow::Action(next_cmd));
            }
        }
        workflows.insert(name.to_owned(), instructions);
    }
    for line in lines {
        let mut part_vec = Vec::new();
        for part in line[1..line.len() - 1].split(",") {
            let ptype = part.chars().nth(0).unwrap();
            let num = part[2..].parse::<usize>().unwrap();
            let p = match ptype {
                'x' => PartType::X(num),
                'm' => PartType::M(num),
                'a' => PartType::A(num),
                's' => PartType::S(num),
                _ => unreachable!(),
            };
            part_vec.push(p);
        }
        parts.push(part_vec);
    }
    (Arc::new(workflows), Arc::new(parts))
}

type Part2Parsed = Arc<HashMap<String, Vec<WorkFlow>>>;
// #[aoc_generator(day19)]
fn parse2(input: &str) -> Part2Parsed {
    let mut workflows = HashMap::new();
    let mut lines = input.lines();
    workflows.insert("A".to_owned(), vec![WorkFlow::Accept]);
    workflows.insert("R".to_owned(), vec![WorkFlow::Reject]);
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut vals = line.split("{");
        let name = vals.next().unwrap();
        let wk = vals.next().unwrap().strip_suffix("}").unwrap();
        let parts = wk.split(",").collect::<Vec<_>>();
        let mut instructions = Vec::new();
        for p in parts {
            let pvals = p.split(":").collect::<Vec<_>>();
            if pvals.len() > 1 {
                let condition = pvals[0];
                let mut first_two = condition.chars().take(2);
                let amt = condition[2..].parse::<usize>().unwrap();
                let p_type = match first_two.next().unwrap() {
                    'x' => PartType::X(amt),
                    'm' => PartType::M(amt),
                    'a' => PartType::A(amt),
                    's' => PartType::S(amt),
                    _ => unreachable!(),
                };

                let cond = match first_two.next().unwrap() {
                    '<' => Cond::Lt(p_type),
                    '>' => Cond::Gt(p_type),
                    _ => unreachable!(),
                };

                let next_cmd = pvals[1].to_owned();
                instructions.push(WorkFlow::Condition(cond, next_cmd));
            } else {
                let next_cmd = pvals[0].to_owned();
                instructions.push(WorkFlow::Action(next_cmd));
            }
        }
        workflows.insert(name.to_owned(), instructions);
    }

    Arc::new(workflows)
}

enum WorkFlow {
    Condition(Cond, String),
    Action(String),
    Accept,
    Reject,
}

#[derive(Debug)]
enum PartType {
    X(usize),
    M(usize),
    A(usize),
    S(usize),
}

impl PartType {
    fn amt(&self) -> usize {
        match self {
            PartType::X(num) => *num,
            PartType::M(num) => *num,
            PartType::A(num) => *num,
            PartType::S(num) => *num,
        }
    }
}

impl PartialOrd for PartType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PartType::X(num1), PartType::X(num2)) => num1.partial_cmp(num2),
            (PartType::M(num1), PartType::M(num2)) => num1.partial_cmp(num2),
            (PartType::A(num1), PartType::A(num2)) => num1.partial_cmp(num2),
            (PartType::S(num1), PartType::S(num2)) => num1.partial_cmp(num2),
            _ => None,
        }
    }
}

impl PartialEq for PartType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PartType::X(num1), PartType::X(num2)) => num1 == num2,
            (PartType::M(num1), PartType::M(num2)) => num1 == num2,
            (PartType::A(num1), PartType::A(num2)) => num1 == num2,
            (PartType::S(num1), PartType::S(num2)) => num1 == num2,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum Cond {
    Lt(PartType),
    Gt(PartType),
}

impl Cond {
    fn eval(&self, part: &Vec<PartType>) -> bool {
        match self {
            Cond::Lt(p) => match p {
                PartType::X(_num) => part[0] < *p,
                PartType::M(_num) => part[1] < *p,
                PartType::A(_num) => part[2] < *p,
                PartType::S(_num) => part[3] < *p,
            },
            Cond::Gt(p) => match p {
                PartType::X(_num) => part[0] > *p,
                PartType::M(_num) => part[1] > *p,
                PartType::A(_num) => part[2] > *p,
                PartType::S(_num) => part[3] > *p,
            },
        }
    }
    fn update_max_min(&self, ranges: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut met_ranges = ranges.clone();
        match self {
            Cond::Lt(p) => match p {
                PartType::X(num) => {
                    met_ranges[0].1 = met_ranges[0].1.min(*num - 1);
                    ranges[0].0 = ranges[0].0.max(*num);
                }
                PartType::M(num) => {
                    met_ranges[1].1 = met_ranges[1].1.min(*num - 1);
                    ranges[1].0 = ranges[1].0.max(*num);
                }
                PartType::A(num) => {
                    met_ranges[2].1 = met_ranges[2].1.min(*num - 1);
                    ranges[2].0 = ranges[2].0.max(*num);
                }
                PartType::S(num) => {
                    met_ranges[3].1 = met_ranges[3].1.min(*num - 1);
                    ranges[3].0 = ranges[3].0.max(*num);
                }
            },
            Cond::Gt(p) => match p {
                PartType::X(num) => {
                    met_ranges[0].0 = met_ranges[0].0.max(*num + 1);
                    ranges[0].1 = ranges[0].1.min(*num);
                }
                PartType::M(num) => {
                    met_ranges[1].0 = met_ranges[1].0.max(*num + 1);
                    ranges[1].1 = ranges[1].1.min(*num);
                }
                PartType::A(num) => {
                    met_ranges[2].0 = met_ranges[2].0.max(*num + 1);
                    ranges[2].1 = ranges[2].1.min(*num);
                }
                PartType::S(num) => {
                    met_ranges[3].0 = met_ranges[3].0.max(*num + 1);
                    ranges[3].1 = ranges[3].1.min(*num);
                }
            },
        }
        met_ranges
    }
}

fn sort_part(part: &Vec<PartType>, workflows: &HashMap<String, Vec<WorkFlow>>) -> usize {
    let mut name = "in";
    loop {
        for wkf in workflows.get(name).unwrap() {
            match wkf {
                WorkFlow::Action(cmd) => {
                    name = cmd;
                    break;
                }
                WorkFlow::Condition(cond, cmd) => {
                    if cond.eval(part) {
                        name = cmd;
                        break;
                    }
                }
                WorkFlow::Accept => {
                    return part.iter().map(|p| p.amt()).sum::<usize>();
                }
                WorkFlow::Reject => {
                    return 0;
                }
            }
        }
    }
}

#[aoc(day19, part1, serial)]
pub fn part_1_serial(input: &str) -> usize {
    let (workflows, parts) = parse(input);
    parts.iter().map(|part| sort_part(part, &workflows)).sum()
}

#[aoc(day19, part2, serial)]
pub fn part_2_serial(input: &str) -> usize {
    let workflows = parse2(input);
    let mut name_stack = VecDeque::new();
    name_stack.push_back(("in", vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)]));
    let mut sum = 0;
    while let Some((name, mut ranges)) = name_stack.pop_front() {
        for wkf in workflows.get(name).unwrap() {
            match wkf {
                WorkFlow::Action(cmd) => {
                    name_stack.push_front((cmd, ranges.clone()));
                }
                WorkFlow::Condition(cond, cmd) => {
                    let met_ranges = cond.update_max_min(&mut ranges);
                    name_stack.push_front((cmd, met_ranges));
                }
                WorkFlow::Accept => {
                    sum += ranges.iter().map(|r| r.1 - r.0 + 1).product::<usize>();
                }
                WorkFlow::Reject => {}
            }
        }
    }
    sum
}
