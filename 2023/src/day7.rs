use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day7, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

fn parse_line(line: &str) -> (Vec<u8>, usize) {
    let mut line = line.trim().split_whitespace();
    let hand = line
        .next()
        .unwrap()
        .chars()
        .map(|c| {
            if c.is_numeric() {
                c.to_digit(10).unwrap() as u8
            } else {
                match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => 0,
                }
            }
        })
        .collect::<Vec<u8>>();
    let bid = line.next().unwrap().parse::<usize>().unwrap();
    (hand, bid)
}

fn parse_line_part2(line: &str) -> (Vec<u8>, usize) {
    let mut line = line.trim().split_whitespace();
    let hand = line
        .next()
        .unwrap()
        .chars()
        .map(|c| {
            if c.is_numeric() {
                c.to_digit(10).unwrap() as u8
            } else {
                match c {
                    'T' => 10,
                    'J' => 1,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => 0,
                }
            }
        })
        .collect::<Vec<u8>>();
    let bid = line.next().unwrap().parse::<usize>().unwrap();
    (hand, bid)
}

fn calc_score_part1(hand: &Vec<u8>) -> usize {
    let mut histogram = [1; 15];
    for x in hand {
        histogram[*x as usize] *= 10;
    }
    histogram[2..].iter().sum()
}

fn calc_score_part2(hand: &Vec<u8>) -> usize {
    let mut histogram = [1; 15];
    for x in hand {
        histogram[*x as usize] += 1;
    }
    let mut max = 0;
    histogram[2..]
        .iter()
        .map(|x| {
            if (*x) > max {
                max = *x
            }
            10_usize.pow(*x)
        })
        .sum::<usize>()
        - 10_usize.pow(max)
        + 10_usize.pow(max + histogram[1])
}

#[aoc(day7, part1, serial)]
pub fn part_1_serial(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|x| {
            let (hand, bid) = parse_line(x);
            let score = calc_score_part1(&hand);
            (score, hand, bid)
        })
        .collect::<Vec<_>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (hand.2 * (i + 1)) as u32)
        .sum()
}

#[aoc(day7, part2, serial)]
pub fn part_2_serial(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|x| {
            let (hand, bid) = parse_line_part2(x);
            let score = calc_score_part2(&hand);
            (score, hand, bid)
        })
        .collect::<Vec<_>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (hand.2 * (i + 1)) as u32)
        .sum()
}

#[AmLocalData]
struct Part1 {
    lines: std::sync::Arc<Vec<String>>,
    start: usize,
    n: usize,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() -> Vec<(usize, Vec<u8>, usize)> {
        let mut hands = vec![];
        for line in self.lines[self.start..].iter().step_by(self.n) {
            let (hand, bid) = parse_line(line);
            hands.push((calc_score_part1(&hand), hand, bid));
        }
        hands
    }
}

#[aoc_generator(day7, part1, am)]
fn parse_input_1_am(input: &str) -> std::sync::Arc<Vec<String>> {
    std::sync::Arc::new(input.lines().map(|x| x.to_string()).collect::<Vec<_>>())
}

#[aoc(day7, part1, am)]
pub fn part_1_am(input: &std::sync::Arc<Vec<String>>) -> u32 {
    let num_threads = WORLD.num_threads_per_pe();
    WORLD.block_on(async move {
        let reqs = (0..num_threads)
            .map(|t| {
                WORLD.exec_am_local(Part1 {
                    lines: input.clone(),
                    start: t,
                    n: num_threads,
                })
            })
            .collect::<Vec<_>>();
        let mut hands = futures::future::join_all(reqs)
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (hand.2 * (i + 1)) as u32)
            .sum()
    })
}

#[AmLocalData]
struct Part2 {
    lines: std::sync::Arc<Vec<String>>,
    start: usize,
    n: usize,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec() -> Vec<(usize, Vec<u8>, usize)> {
        let mut hands = vec![];
        for line in self.lines[self.start..].iter().step_by(self.n) {
            let (hand, bid) = parse_line_part2(line);
            hands.push((calc_score_part2(&hand), hand, bid));
        }
        hands
    }
}

#[aoc_generator(day7, part2, am)]
fn parse_input_2_am(input: &str) -> std::sync::Arc<Vec<String>> {
    std::sync::Arc::new(input.lines().map(|x| x.to_string()).collect::<Vec<_>>())
}

#[aoc(day7, part2, am)]
pub fn part_2_am(input: &std::sync::Arc<Vec<String>>) -> u32 {
    let num_threads = WORLD.num_threads_per_pe();
    WORLD.block_on(async move {
        let reqs = (0..num_threads)
            .map(|t| {
                WORLD.exec_am_local(Part2 {
                    lines: input.clone(),
                    start: t,
                    n: num_threads,
                })
            })
            .collect::<Vec<_>>();
        let mut hands = futures::future::join_all(reqs)
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (hand.2 * (i + 1)) as u32)
            .sum()
    })
}
