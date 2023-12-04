use lamellar::active_messaging::prelude::*;

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[AmData(Debug)]
struct Part1 {
    line: String,
}

#[am]
impl LamellarAm for Part1 {
    async fn exec() -> u32 {
        let mut valid = true;
        let line = self.line.split(":").collect::<Vec<&str>>();
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
}

#[AmData(Debug)]
struct Part2 {
    line: String,
}

#[am]
impl LamellarAm for Part2 {
    async fn exec() -> u32 {
        let mut valid = true;
        let line = self.line.split(":").collect::<Vec<&str>>();
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
}

pub fn part_1(world: &LamellarWorld) {
    let f = File::open("inputs/day2.txt").unwrap();
    let reqs = BufReader::new(&f)
        .lines()
        .into_iter()
        .map(|line| {
            world.exec_am_local(Part1 {
                line: line.unwrap(),
            })
        })
        .collect::<Vec<_>>();
    let sum: u32 = world.block_on(futures::future::join_all(reqs)).iter().sum();
    println!("Sum: {sum}");
}

pub fn part_1_task_group(world: &LamellarWorld) {
    let f = File::open("inputs/day2.txt").unwrap();
    let my_pe = world.my_pe();
    let mut tg = typed_am_group! {Part1, world};
    for line in BufReader::new(&f).lines() {
        tg.add_am_pe(
            my_pe,
            Part1 {
                line: line.unwrap(),
            },
        )
    }
    let sum: u32 = world
        .block_on(tg.exec())
        .iter()
        .map(|x| {
            if let AmGroupResult::Pe(pe, val) = x {
                *val
            } else {
                0
            }
        })
        .sum();
    println!("Sum: {sum}");
}

pub fn part_2(world: &LamellarWorld) {
    let f = File::open("inputs/day2.txt").unwrap();
    let reqs = BufReader::new(&f)
        .lines()
        .into_iter()
        .map(|line| {
            world.exec_am_local(Part2 {
                line: line.unwrap(),
            })
        })
        .collect::<Vec<_>>();
    let sum: u32 = world.block_on(futures::future::join_all(reqs)).iter().sum();
    println!("Sum: {sum}");
}

pub fn part_2_task_group(world: &LamellarWorld) {
    let f = File::open("inputs/day2.txt").unwrap();
    let my_pe = world.my_pe();
    let mut tg = typed_am_group! {Part2, world};
    for line in BufReader::new(&f).lines() {
        tg.add_am_pe(
            my_pe,
            Part2 {
                line: line.unwrap(),
            },
        )
    }
    let sum: u32 = world
        .block_on(tg.exec())
        .iter()
        .map(|x| {
            if let AmGroupResult::Pe(pe, val) = x {
                *val
            } else {
                0
            }
        })
        .sum();
    println!("Sum: {sum}");
}
