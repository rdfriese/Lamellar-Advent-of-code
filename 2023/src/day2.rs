use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day2, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn process_line_part1(line: &str) -> u32 {
    let mut valid = true;
    let line = line.split(":").collect::<Vec<&str>>();
    for set in line[1].split(";") {
        for color in set.split(",") {
            let mut iter = color.trim().split_ascii_whitespace();
            let amount = iter.next().unwrap().parse::<u32>().unwrap();
            let color = iter.next().unwrap();
            let max = match color {
                "blue" => MAX_BLUE,
                "red" => MAX_RED,
                "green" => MAX_GREEN,
                _ => 0,
            };
            if amount > max {
                valid = false;
                break;
            }
        }
    }
    if valid {
        line[0]
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap()
    } else {
        0
    }
}

fn process_line_part2(line: &str) -> u32 {
    let line = line.split(":").collect::<Vec<&str>>();
    let mut min_blue = 0;
    let mut min_red = 0;
    let mut min_green = 0;
    for set in line[1].split(";") {
        for color in set.split(",") {
            let mut iter = color.trim().split_ascii_whitespace();
            let amount = iter.next().unwrap().parse::<u32>().unwrap();
            let color = iter.next().unwrap();
            match color {
                "blue" => {
                    if amount > min_blue {
                        min_blue = amount
                    }
                }
                "red" => {
                    if amount > min_red {
                        min_red = amount
                    }
                }
                "green" => {
                    if amount > min_green {
                        min_green = amount
                    }
                }
                _ => {}
            }
        }
    }
    min_blue * min_red * min_green
}

#[aoc(day2, part1, serial)]
pub fn part_1_serial(input: &str) -> u32 {
    input.lines().map(|line| process_line_part1(line)).sum()
}

#[aoc(day2, part2, serial)]
pub fn part_2_serial(input: &str) -> u32 {
    input.lines().map(|line| process_line_part2(line)).sum()
}

#[aoc_generator(day2, part1, am)]
fn parse_input_1_am(input: &str) -> Darc<Vec<String>> {
    Darc::new(
        &*WORLD,
        input.lines().map(|x| x.to_string()).collect::<Vec<_>>(),
    )
    .unwrap()
}

#[AmData(Debug)]
struct Part1 {
    lines: Darc<Vec<String>>,
    start: usize,
    n: usize,
}

#[am]
impl LamellarAm for Part1 {
    async fn exec() -> u32 {
        self.lines[self.start..]
            .iter()
            .step_by(self.n)
            .map(|line| process_line_part1(line))
            .sum::<u32>()
    }
}

#[aoc(day2, part1, am)]
pub fn part_1_am(input: &Darc<Vec<String>>) -> u32 {
    let num_threads = WORLD.num_threads_per_pe();
    let reqs = (0..num_threads)
        .map(|t| {
            WORLD.exec_am_local(Part1 {
                lines: input.clone(),
                start: t,
                n: num_threads,
            })
        })
        .collect::<Vec<_>>();
    WORLD.block_on(futures::future::join_all(reqs)).iter().sum()
}

#[aoc_generator(day2, part2, am)]
fn parse_input_2_am(input: &str) -> Darc<Vec<String>> {
    Darc::new(
        &*WORLD,
        input.lines().map(|x| x.to_string()).collect::<Vec<_>>(),
    )
    .unwrap()
}

#[AmData(Debug)]
struct Part2 {
    lines: Darc<Vec<String>>,
    start: usize,
    n: usize,
}

#[am]
impl LamellarAm for Part2 {
    async fn exec() -> u32 {
        self.lines[self.start..]
            .iter()
            .step_by(self.n)
            .map(|line| process_line_part2(&line))
            .sum::<u32>()
    }
}

#[aoc(day2, part2, am)]
pub fn part_2(input: &Darc<Vec<String>>) -> u32 {
    let num_threads = WORLD.num_threads_per_pe();
    let reqs = (0..num_threads)
        .map(|t| {
            WORLD.exec_am_local(Part2 {
                lines: input.clone(),
                start: t,
                n: num_threads,
            })
        })
        .collect::<Vec<_>>();
    WORLD.block_on(futures::future::join_all(reqs)).iter().sum()
}
