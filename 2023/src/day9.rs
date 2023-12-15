use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

#[aoc_generator(day9)]
fn parse_part1(input: &str) -> Darc<Vec<Vec<i32>>> {
    Darc::new(
        &*WORLD,
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap()
}
// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day9, part1, A_INIT_WORLD)]
pub fn part_1(_: &Darc<Vec<Vec<i32>>>) -> usize {
    WORLD.num_pes()
}

fn part1_sum(data: &[Vec<i32>]) -> isize {
    data.iter()
        .map(|d| {
            let mut sum = d[d.len() - 1];
            let mut next = d
                .as_slice()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect::<Vec<i32>>();
            while next.iter().any(|x| *x != 0) {
                sum += next[next.len() - 1];
                next = next
                    .as_slice()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<i32>>();
            }
            sum as isize
        })
        .sum()
}

#[aoc(day9, part1, serial)]
pub fn part_1_serial(data: &Darc<Vec<Vec<i32>>>) -> isize {
    part1_sum(data)
}

pub fn part2_sum(data: &[Vec<i32>]) -> isize {
    data.iter()
        .map(|d| {
            let mut firsts = vec![d[0]];
            let mut next = d
                .as_slice()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect::<Vec<i32>>();
            while next.iter().any(|x| *x != 0) {
                firsts.push(next[0]);
                next = next
                    .as_slice()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<i32>>();
            }
            firsts.iter().rev().fold(0, |acc, x| x - acc) as isize
        })
        .sum()
}

#[aoc(day9, part2, serial)]
pub fn part_2_serial(data: &Darc<Vec<Vec<i32>>>) -> isize {
    part2_sum(data)
}

#[AmData]
struct Part1 {
    data: Darc<Vec<Vec<i32>>>,
    start: usize,
    length: usize,
}

#[am]
impl LamellarAm for Part1 {
    async fn exec() -> isize {
        part1_sum(&self.data[self.start..self.start + self.length])
    }
}

#[aoc(day9, part1, am)]
pub fn part_1_am(data: &Darc<Vec<Vec<i32>>>) -> isize {
    let num_threads = WORLD.num_threads_per_pe();
    let num_lines_per_thread = std::cmp::max(1, data.len() / num_threads); //for the test inputs
    let reqs = data.chunks(num_lines_per_thread).enumerate().map(|(i, d)| {
        WORLD.exec_am_local(Part1 {
            data: data.clone(),
            start: i * num_lines_per_thread,
            length: d.len(),
        })
    });
    WORLD
        .block_on(futures::future::join_all(reqs))
        .iter()
        .sum::<isize>()
}

#[AmData]
struct Part2 {
    data: Darc<Vec<Vec<i32>>>,
    start: usize,
    length: usize,
}

#[am]
impl LamellarAm for Part2 {
    async fn exec() -> isize {
        part2_sum(&self.data[self.start..self.start + self.length])
    }
}

#[aoc(day9, part2, am)]
pub fn part_2_am(data: &Darc<Vec<Vec<i32>>>) -> isize {
    let num_threads = WORLD.num_threads_per_pe();
    let num_lines_per_thread = std::cmp::max(1, data.len() / num_threads); //for the test inputs
    let reqs = data.chunks(num_lines_per_thread).enumerate().map(|(i, d)| {
        WORLD.exec_am_local(Part2 {
            data: data.clone(),
            start: i * num_lines_per_thread,
            length: d.len(),
        })
    });
    WORLD
        .block_on(futures::future::join_all(reqs))
        .iter()
        .sum::<isize>()
}
