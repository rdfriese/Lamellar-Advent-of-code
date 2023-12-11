use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

fn process_line_part1(line: &str) -> u32 {
    let first = line
        .chars()
        .filter_map(|x| x.to_digit(10))
        .next()
        .expect("no number found");
    let second = line
        .chars()
        .rev()
        .filter_map(|x| x.to_digit(10))
        .next()
        .unwrap_or(first);
    first * 10 + second
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process_line_part2(line: &str) -> u32 {
    let mut idx = line.len() - 1;
    let mut first = line
        .char_indices()
        .filter_map(|(i, c)| {
            idx = i;
            c.to_digit(10)
        })
        .next();

    let mut min_i = idx;
    for (i, digit) in DIGITS.iter().enumerate() {
        if let Some(f) = line[..=idx].find(digit) {
            if f < min_i {
                min_i = f;
                first = Some(i as u32 + 1);
            }
        }
    }

    let mut second = line
        .char_indices()
        .rev()
        .filter_map(|(i, c)| {
            idx = i;
            c.to_digit(10)
        })
        .next();
    if second.is_none() {
        idx = 0;
    }
    let mut max_i = 0;
    for (i, digit) in DIGITS.iter().enumerate() {
        if let Some(f) = line[idx..].rfind(digit) {
            if f > max_i {
                max_i = f;
                second = Some(i as u32 + 1);
            }
        }
    }
    match (first, second) {
        (Some(x), Some(y)) => x * 10 + y,
        (Some(x), None) => x,
        (None, Some(x)) => x,
        (None, None) => 0,
    }
}

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day1, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

#[aoc(day1, part1, serial)]
pub fn part_1_serial(input: &str) -> u32 {
    input.lines().map(|line| process_line_part1(line)).sum()
}

#[aoc(day1, part2, serial)]
pub fn part_2_serial(input: &str) -> u32 {
    input.lines().map(|line| process_line_part2(line)).sum()
}

#[aoc_generator(day1, part1, am)]
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
            .map(|line| process_line_part1(&line))
            .sum::<u32>()
    }
}

#[aoc(day1, part1, am)]
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

#[aoc_generator(day1, part2, am)]
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

#[aoc(day1, part2, am)]
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
