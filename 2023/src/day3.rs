use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

use std::{
    str,
    sync::atomic::{AtomicU32, Ordering},
};

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day3, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

fn check_left(line: &[u8], i: usize) -> u32 {
    let mut start_i = i;
    while start_i > 0 {
        if !(line[start_i - 1] as char).is_numeric() {
            break;
        }
        start_i -= 1;
    }
    str::from_utf8(&line[start_i..i])
        .expect("only ascii")
        .parse::<u32>()
        .unwrap_or(0)
}

fn check_right(line: &[u8], i: usize) -> u32 {
    let mut end_i = i;
    while end_i < line.len() - 1 {
        if !(line[end_i + 1] as char).is_numeric() {
            break;
        }
        end_i += 1;
    }
    str::from_utf8(&line[i + 1..=end_i])
        .expect("only ascii")
        .parse::<u32>()
        .ok()
        .unwrap_or(0)
}

// this is when we are checking above or below
fn check_center(line: &[u8], i: usize) -> (u32, u32) {
    if !(line[i] as char).is_numeric() {
        //not one long number so check left and right diagonals
        return (check_left(line, i), check_right(line, i));
    } else {
        let mut start_i: usize = i;
        let mut end_i = i;
        while start_i > 0 {
            if !(line[start_i - 1] as char).is_numeric() {
                break;
            }
            start_i -= 1;
        }
        while end_i < line.len() - 1 {
            if !(line[end_i + 1] as char).is_numeric() {
                break;
            }
            end_i += 1;
        }
        (
            str::from_utf8(&line[start_i..=end_i])
                .expect("only ascii")
                .parse::<u32>()
                .ok()
                .unwrap_or(0),
            0,
        )
    }
}

fn process_line_part1(line: usize, schematic: &Vec<Vec<u8>>) -> u32 {
    let mut my_sum = 0;
    for (i, c) in schematic[line]
        .iter()
        .enumerate()
        .map(|(i, c)| (i, *c as char))
    {
        if !c.is_numeric() && c != '.' {
            // check line above me
            if line != 0 {
                let center = check_center(&schematic[line - 1], i);
                my_sum += center.0 + center.1;
            }
            // check my line
            my_sum += check_left(&schematic[line], i) + check_right(&schematic[line], i);
            // check below me
            if line != schematic.len() - 1 {
                let center = check_center(&schematic[line + 1], i);
                my_sum += center.0 + center.1;
            }
        }
    }
    my_sum
}

fn process_line_part2(line: usize, schematic: &Vec<Vec<u8>>) -> u32 {
    let mut my_sum = 0;
    for (i, c) in schematic[line]
        .iter()
        .enumerate()
        .map(|(i, c)| (i, *c as char))
    {
        if !c.is_numeric() && c != '.' {
            let mut nums = vec![];
            // check line above me
            if line != 0 {
                let center = check_center(&schematic[line - 1], i);
                if center.0 != 0 {
                    nums.push(center.0);
                }
                if center.1 != 0 {
                    nums.push(center.1);
                }
            }
            // check my line
            let left = check_left(&schematic[line], i);
            if left != 0 {
                nums.push(left);
            }
            let right = check_right(&schematic[line], i);
            if right != 0 {
                nums.push(right);
            }
            // check below me
            if line != schematic.len() - 1 {
                let center = check_center(&schematic[line + 1], i);
                if center.0 != 0 {
                    nums.push(center.0);
                }
                if center.1 != 0 {
                    nums.push(center.1);
                }
            }
            if nums.len() == 2 {
                my_sum += nums[0] * nums[1];
            }
        }
    }
    my_sum
}

#[aoc_generator(day3, serial)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.to_string().into_bytes())
        .collect::<Vec<_>>()
}

#[aoc(day3, part1, serial)]
pub fn part_1_serial(input: &Vec<Vec<u8>>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(i, _)| process_line_part1(i, input))
        .sum()
}

#[aoc(day3, part2, serial)]
pub fn part_2_serial(input: &Vec<Vec<u8>>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(i, _)| process_line_part2(i, input))
        .sum()
}

// we can also create active messages that only execute locally
#[AmData(Debug)]
struct Part1 {
    // a darc is lamellar construct for a "distributed Arc"
    schematic: Darc<Vec<Vec<u8>>>, //store as bytes for easy indexing, assuming input is only ascii
    start: usize,
    n: usize,
    sum: Darc<AtomicU32>,
}

#[am]
impl LamellarAm for Part1 {
    async fn exec() {
        self.sum.fetch_add(
            self.schematic[self.start..]
                .iter()
                .enumerate()
                .step_by(self.n)
                .map(|(i, _)| process_line_part1(i + self.start, &self.schematic))
                .sum::<u32>(),
            Ordering::Relaxed,
        );
    }
}

#[aoc_generator(day3, part1, am)]
fn parse_am(input: &str) -> Darc<Vec<Vec<u8>>> {
    Darc::new(
        WORLD.team(),
        input
            .lines()
            .map(|line| line.to_string().into_bytes())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[aoc(day3, part1, am)]
pub fn part_1_am(input: &Darc<Vec<Vec<u8>>>) -> u32 {
    let sum = Darc::new(WORLD.team(), AtomicU32::new(0)).unwrap();
    let num_threads = WORLD.num_threads_per_pe();
    for t in 0..num_threads {
        WORLD.exec_am_local(Part1 {
            schematic: input.clone(),
            start: t,
            n: num_threads,
            sum: sum.clone(),
        });
    }
    WORLD.wait_all();
    sum.load(Ordering::SeqCst)
}

#[AmLocalData(Debug)]
struct Part2 {
    schematic: Darc<Vec<Vec<u8>>>, //store as bytes for easy indexing, assuming input is only ascii
    start: usize,
    n: usize,
    sum: Darc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec() {
        self.sum.fetch_add(
            self.schematic[self.start..]
                .iter()
                .enumerate()
                .step_by(self.n)
                .map(|(i, _)| process_line_part2(i + self.start, &self.schematic))
                .sum::<u32>(),
            Ordering::Relaxed,
        );
    }
}

#[aoc_generator(day3, part2, am)]
fn parse_am_2(input: &str) -> Darc<Vec<Vec<u8>>> {
    Darc::new(
        WORLD.team(),
        input
            .lines()
            .map(|line| line.to_string().into_bytes())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[aoc(day3, part2, am)]
pub fn part_2_am(input: &Darc<Vec<Vec<u8>>>) -> u32 {
    let sum = Darc::new(WORLD.team(), AtomicU32::new(0)).unwrap();
    let num_threads = WORLD.num_threads_per_pe();
    for t in 0..num_threads {
        WORLD.exec_am_local(Part2 {
            schematic: input.clone(),
            start: t,
            n: num_threads,
            sum: sum.clone(),
        });
    }
    WORLD.wait_all();
    sum.load(Ordering::SeqCst)
}
