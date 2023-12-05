use lamellar::active_messaging::prelude::*;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[AmData(Debug)]
struct Part1 {
    line: String,
}

#[am]
impl LamellarAm for Part1 {
    async fn exec() -> u32 {
        let first = self
            .line
            .chars()
            .filter_map(|x| x.to_digit(10))
            .next()
            .expect("no number found");
        let second = self
            .line
            .chars()
            .rev()
            .filter_map(|x| x.to_digit(10))
            .next()
            .unwrap_or(first);
        first * 10 + second
    }
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[AmData(Debug)]
struct Part2 {
    line: String,
}

#[am]
impl LamellarAm for Part2 {
    async fn exec() -> u32 {
        let mut idx = self.line.len() - 1;
        let mut first = self
            .line
            .char_indices()
            .filter_map(|(i, c)| {
                idx = i;
                c.to_digit(10)
            })
            .next();

        let mut min_i = idx;
        for (i, digit) in DIGITS.iter().enumerate() {
            if let Some(f) = self.line[..=idx].find(digit) {
                if f < min_i {
                    min_i = f;
                    first = Some(i as u32 + 1);
                }
            }
        }

        let mut second = self
            .line
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
            if let Some(f) = self.line[idx..].rfind(digit) {
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
}

pub fn part_1(world: &LamellarWorld) {
    let f = File::open("inputs/day1.txt").unwrap();
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
    let f = File::open("inputs/day1.txt").unwrap();
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
            if let AmGroupResult::Pe(_pe, val) = x {
                *val
            } else {
                0
            }
        })
        .sum();
    println!("Sum: {sum}");
}

pub fn part_2(world: &LamellarWorld) {
    let f = File::open("inputs/day1.txt").unwrap();
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
    let f = File::open("inputs/day1.txt").unwrap();
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
            if let AmGroupResult::Pe(_pe, val) = x {
                *val
            } else {
                0
            }
        })
        .sum();
    println!("Sum: {sum}");
}
