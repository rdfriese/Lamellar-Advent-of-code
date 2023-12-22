use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
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
#[aoc(day20, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

static LOW: bool = true;
static HIGH: bool = false;

static OFF: bool = false;
static ON: bool = true;

type Part1Parsed = (
    HashMap<String, Component>,
    BTreeMap<String, bool>,
    HashMap<String, HashMap<String, bool>>,
);
// #[aoc_generator(day19)]
fn parse(input: &str) -> Part1Parsed {
    let mut components = HashMap::new();
    // components.insert("output".to_owned(), Component::Output(vec![]));
    let mut flip_flops = BTreeMap::new(); //key is the index in the input>
    let mut conjunctions = HashMap::new();
    for line in input.lines() {
        let mut vals = line.split(" -> ");
        let name = vals.next().unwrap();
        match name.chars().nth(0).unwrap() {
            'b' => {
                let connections = vals
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|s| s.trim().to_owned())
                    .collect();

                components.insert("broadcast".to_owned(), Component::Broadcast(connections));
            }
            '%' => {
                let connections = vals
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|s| s.trim().to_owned())
                    .collect();
                components.insert(name[1..].to_owned(), Component::FlipFlop(connections));
                flip_flops.insert(name[1..].to_owned(), OFF);
            }
            '&' => {
                let connections = vals
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|s| s.trim().to_owned())
                    .collect();
                components.insert(name[1..].to_owned(), Component::Conjunction(connections));
                conjunctions.insert(name[1..].to_owned(), HashMap::new());
            }
            _ => unreachable!(),
        }
    }

    let mut add_connections = vec![];
    for (name, component) in &components {
        for conn in component.connections() {
            if let Some(comp) = components.get(conn.as_str()) {
                if let Component::Conjunction(_) = comp {
                    conjunctions
                        .get_mut(conn.as_str())
                        .unwrap()
                        .insert(name.to_owned(), LOW);
                }
            } else {
                add_connections.push(conn.to_owned());
            }
        }
    }
    for conn in add_connections.drain(..) {
        components.insert(conn, Component::Output(vec![]));
    }
    (components, flip_flops, conjunctions)
}

#[derive(Debug, Clone)]
enum Component {
    Broadcast(Vec<String>),
    FlipFlop(Vec<String>),
    Conjunction(Vec<String>),
    Output(Vec<String>),
}

impl Component {
    fn connections(&self) -> &Vec<String> {
        match self {
            Component::Broadcast(connections) => connections,
            Component::FlipFlop(connections) => connections,
            Component::Conjunction(connections) => connections,
            Component::Output(connections) => connections,
        }
    }
}

#[aoc(day20, part1, serial)]
pub fn part_1_serial(input: &str) -> usize {
    let (components, mut flip_flops, mut conjunctions) = parse(input);

    let mut stack = VecDeque::new();
    let mut high_pulses = 0;
    let mut low_pulses = 0;
    let mut button_pushes = 0;
    while (flip_flops.iter().filter(|(_, v)| **v == ON).count() > 0 || button_pushes == 0)
        && button_pushes < 1000
    {
        stack.push_back(("broadcast", LOW));
        low_pulses += 1;
        while let Some((name, pulse)) = stack.pop_front() {
            match components.get(name).unwrap() {
                Component::Broadcast(cmds) => {
                    if pulse == HIGH {
                        high_pulses += cmds.len();
                    } else {
                        low_pulses += cmds.len();
                    }
                    for cmd in cmds {
                        match components.get(cmd).unwrap() {
                            Component::Conjunction(_cmds) => {
                                *conjunctions.get_mut(cmd).unwrap().get_mut(name).unwrap() = pulse;
                            }
                            _ => {}
                        }
                        stack.push_back((cmd, pulse));
                    }
                }
                Component::FlipFlop(cmds) => {
                    let state = flip_flops.get_mut(name).unwrap();
                    if pulse == LOW {
                        let new_pulse = if *state == OFF {
                            *state = ON;
                            HIGH
                        } else {
                            *state = OFF;
                            LOW
                        };
                        if new_pulse == HIGH {
                            high_pulses += cmds.len();
                        } else {
                            low_pulses += cmds.len();
                        }
                        for cmd in cmds {
                            match components.get(cmd).unwrap() {
                                Component::Conjunction(_cmds) => {
                                    *conjunctions.get_mut(cmd).unwrap().get_mut(name).unwrap() =
                                        new_pulse;
                                }
                                _ => {}
                            }
                            stack.push_back((cmd, new_pulse));
                        }
                    }
                }
                Component::Conjunction(cmds) => {
                    let the_pulse =
                        if conjunctions.get(name).unwrap().values().all(|x| *x == HIGH) == true {
                            LOW
                        } else {
                            HIGH
                        };
                    if the_pulse == HIGH {
                        high_pulses += cmds.len();
                    } else {
                        low_pulses += cmds.len();
                    }
                    for cmd in cmds {
                        match components.get(cmd).unwrap() {
                            Component::Conjunction(_cmds) => {
                                *conjunctions.get_mut(cmd).unwrap().get_mut(name).unwrap() =
                                    the_pulse;
                            }
                            _ => {}
                        }
                        stack.push_back((cmd, the_pulse));
                    }
                }
                Component::Output(_) => {}
            }
        }
        button_pushes += 1;
    }
    let num_cycles = 1000 / button_pushes;
    (high_pulses * num_cycles) * (low_pulses * num_cycles)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

#[aoc(day20, part2, serial)]
pub fn part_2_serial(input: &str) -> usize {
    let (components, mut flip_flops, mut conjunctions) = parse(input);

    //based on manually looking at input we see something like:
    // gs  vg zf kd
    //  \  |  |  /
    //   \  ||  /
    //      rg
    //      |
    //      rx
    // so we need to find the periods for each of the four inputs to rq then find least common multiple

    let names_of_interest = vec!["gs", "vg", "zf", "kd"]; // these are all conjunctions
    let mut components_we_care_about = HashMap::new();

    let mut stack = VecDeque::new();
    let mut button_pushes = 0;
    while components_we_care_about.len() < names_of_interest.len() {
        button_pushes += 1;
        stack.push_back(("broadcast", LOW));
        while let Some((name, pulse)) = stack.pop_front() {
            match components.get(name).unwrap() {
                Component::Broadcast(cmds) => {
                    for cmd in cmds {
                        match components.get(cmd).unwrap() {
                            Component::Conjunction(_cmds) => {
                                *conjunctions.get_mut(cmd).unwrap().get_mut(name).unwrap() = pulse;
                            }
                            _ => {}
                        }
                        stack.push_back((cmd, pulse));
                    }
                }
                Component::FlipFlop(cmds) => {
                    let state = flip_flops.get_mut(name).unwrap();
                    if pulse == LOW {
                        let new_pulse = if *state == OFF {
                            *state = ON;
                            HIGH
                        } else {
                            *state = OFF;
                            LOW
                        };
                        for cmd in cmds {
                            match components.get(cmd).unwrap() {
                                Component::Conjunction(_cmds) => {
                                    *conjunctions.get_mut(cmd).unwrap().get_mut(name).unwrap() =
                                        new_pulse;
                                }
                                _ => {}
                            }
                            stack.push_back((cmd, new_pulse));
                        }
                    }
                }
                Component::Conjunction(cmds) => {
                    let the_pulse =
                        if conjunctions.get(name).unwrap().values().all(|x| *x == HIGH) == true {
                            LOW
                        } else {
                            HIGH
                        };
                    if the_pulse == HIGH {
                        if names_of_interest.contains(&name) {
                            components_we_care_about
                                .entry(name)
                                .or_insert(button_pushes);
                        }
                    }

                    for cmd in cmds {
                        match components.get(cmd).unwrap() {
                            Component::Conjunction(_cmds) => {
                                *conjunctions.get_mut(cmd).unwrap().get_mut(name).unwrap() =
                                    the_pulse;
                            }
                            _ => {}
                        }
                        stack.push_back((cmd, the_pulse));
                    }
                }
                Component::Output(_) => {}
            }
        }
    }
    components_we_care_about
        .iter()
        .fold(1, |acc, (_, v)| lcm(acc, *v))
}
