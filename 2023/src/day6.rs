use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;

fn parse_num_list(line: &str) -> Vec<f64> {
    line.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() as f64)
        .collect::<Vec<f64>>()
}

fn parse_num(line: &str) -> f64 {
    //remove all white space and parse the number
    line.chars()
        .filter(|x| !x.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap() as f64
}

static ACCEL: f64 = 1.0; //(1mm/ms^2)

#[AmData]
struct Part1 {
    time: f64,
    distance: f64,
}

//use quadratic formula to get the roots
// x = -b +- sqrt(b^2 - 4ac) / 2a
fn return_roots(time: f64, distance: f64) -> (u32, u32) {
    let a = ACCEL;
    let b = -time;
    let c = distance + 0.0001; //cheap hack to ensure we are strictly greater than the distance -- not terribly robust numerically but good enough for the inputs provided
    let discriminant = b * b - 4.0 * a * c;
    (
        ((-b + discriminant.sqrt()) / (2.0 * a)).floor() as u32,
        ((-b - discriminant.sqrt()) / (2.0 * a)).ceil() as u32,
    )
}

#[am]
impl LamellarAm for Part1 {
    async fn exec() -> u32 {
        let roots = return_roots(self.time, self.distance);
        roots.0.abs_diff(roots.1) + 1
    }
}

#[aoc_generator(day6, part1)]
fn parse_part1(input: &str) -> (Vec<f64>, Vec<f64>) {
    let mut lines = input.lines();
    let times = parse_num_list(
        lines
            .next()
            .expect("properly formatted input")
            .split(":")
            .last()
            .expect("properly formatted input"),
    );
    let distances = parse_num_list(
        lines
            .next()
            .expect("properly formatted input")
            .split(":")
            .last()
            .expect("properly formatted input"),
    );
    (times, distances)
}

#[aoc_generator(day6, part2)]
fn parse_part2(input: &str) -> (f64, f64) {
    let mut lines = input.lines();
    let time = parse_num(
        lines
            .next()
            .expect("properly formatted input")
            .split(":")
            .last()
            .expect("properly formatted input"),
    );
    let distance = parse_num(
        lines
            .next()
            .expect("properly formatted input")
            .split(":")
            .last()
            .expect("properly formatted input"),
    );
    (time, distance)
}

#[aoc(day6, part1, A_INIT_WORLD)]
pub fn part_1(_input: &(Vec<f64>, Vec<f64>)) -> u32 {
    WORLD.num_pes() as u32
}

#[aoc(day6, part1, serial)]
pub fn part_1_serial(input: &(Vec<f64>, Vec<f64>)) -> u32 {
    input
        .0
        .iter()
        .zip(input.1.iter())
        .map(|(&time, &distance)| {
            let roots = return_roots(time, distance);
            roots.0.abs_diff(roots.1) + 1
        })
        .product()
}

#[aoc(day6, part1, am)]
pub fn part_1_am(input: &(Vec<f64>, Vec<f64>)) -> u32 {
    let reqs = input
        .0
        .iter()
        .zip(input.1.iter())
        .map(|(&time, &distance)| {
            WORLD.exec_am_local(Part1 {
                time: time,
                distance: distance,
            })
        })
        .collect::<Vec<_>>();
    WORLD
        .block_on(futures::future::join_all(reqs))
        .iter()
        .product::<u32>()
}

#[aoc(day6, part2, serial)]
pub fn part_2_serial((time, distance): &(f64, f64)) -> u32 {
    let roots = return_roots(*time, *distance);
    roots.0.abs_diff(roots.1) + 1
}

#[aoc(day6, part2, am)]
pub fn part_2_am((time, distance): &(f64, f64)) -> u32 {
    WORLD.block_on(WORLD.exec_am_local(Part1 {
        time: *time,
        distance: *distance,
    }))
}
