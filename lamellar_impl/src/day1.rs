use crate::WORLD;
use aoc_runner_derive::aoc;
use lamellar::active_messaging::prelude::*;

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

#[AmData(Debug)]
struct Part1 {
    line: String,
}

#[am]
impl LamellarAm for Part1 {
    async fn exec() -> u32 {
        process_line_part1(&self.line)
    }
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

#[AmData(Debug)]
struct Part2 {
    line: String,
}

#[am]
impl LamellarAm for Part2 {
    async fn exec() -> u32 {
        process_line_part2(&self.line)
    }
}

#[aoc(day1, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

#[aoc(day1, part1, serial)]
pub fn part_1_serial(input: &str) -> u32 {
    input.lines().map(|line| process_line_part1(line)).sum()
}

#[aoc(day1, part1, am)]
pub fn part_1_am(input: &str) -> u32 {
    let reqs = input
        .lines()
        .map(|line| {
            WORLD.exec_am_local(Part1 {
                line: line.to_string(),
            })
        })
        .collect::<Vec<_>>();
    WORLD.block_on(futures::future::join_all(reqs)).iter().sum()
}

#[aoc(day1, part1, am_group)]
pub fn part_1_am_group(input: &str) -> u32 {
    let my_pe = WORLD.my_pe();
    let mut tg = typed_am_group! {Part1, WORLD.team()};
    for line in input.lines() {
        tg.add_am_pe(
            my_pe,
            Part1 {
                line: line.to_string(),
            },
        )
    }
    WORLD
        .block_on(tg.exec())
        .iter()
        .map(|x| {
            if let AmGroupResult::Pe(_pe, val) = x {
                *val
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day1, part2, serial)]
pub fn part_2_serial(input: &str) -> u32 {
    input.lines().map(|line| process_line_part2(line)).sum()
}

#[aoc(day1, part2, am)]
pub fn part_2(input: &str) -> u32 {
    let reqs = input
        .lines()
        .map(|line| {
            WORLD.exec_am_local(Part2 {
                line: line.to_string(),
            })
        })
        .collect::<Vec<_>>();
    WORLD.block_on(futures::future::join_all(reqs)).iter().sum()
}

#[aoc(day1, part2, am_group)]
pub fn part_2_am_group(input: &str) -> u32 {
    let my_pe = WORLD.my_pe();
    let mut tg = typed_am_group! {Part2, WORLD.team()};
    for line in input.lines() {
        tg.add_am_pe(
            my_pe,
            Part2 {
                line: line.to_string(),
            },
        )
    }
    WORLD
        .block_on(tg.exec())
        .iter()
        .map(|x| {
            if let AmGroupResult::Pe(_pe, val) = x {
                *val
            } else {
                0
            }
        })
        .sum()
}
