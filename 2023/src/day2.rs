use crate::WORLD;
use aoc_runner_derive::aoc;
use lamellar::active_messaging::prelude::*;

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

#[aoc(day2, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

#[aoc(day2, part1, serial)]
pub fn part_1_serial(input: &str) -> u32 {
    input.lines().map(|line| process_line_part1(line)).sum()
}

#[aoc(day2, part1, am)]
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

#[aoc(day2, part1, am_group)]
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

#[aoc(day2, part2, serial)]
pub fn part_2_serial(input: &str) -> u32 {
    input.lines().map(|line| process_line_part2(line)).sum()
}

#[aoc(day2, part2, am)]
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

#[aoc(day2, part2, am_group)]
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
