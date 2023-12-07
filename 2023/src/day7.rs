use crate::WORLD;
use aoc_runner_derive::aoc;
use lamellar::active_messaging::prelude::*;

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

#[AmLocalData]
struct Part1 {
    line: String,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() -> (usize, Vec<u8>, usize) {
        let (hand, bid) = parse_line(&self.line);
        (calc_score_part1(&hand), hand, bid)
    }
}

#[AmLocalData]
struct Part2 {
    line: String,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec() -> (usize, Vec<u8>, usize) {
        let (hand, bid) = parse_line_part2(&self.line);
        (calc_score_part2(&hand), hand, bid)
    }
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

#[aoc(day7, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
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

#[aoc(day7, part1, am)]
pub fn part_1_am(input: &str) -> u32 {
    WORLD.block_on(async move {
        let reqs = input
            .lines()
            .map(|x| {
                WORLD.exec_am_local(Part1 {
                    line: x.to_string(),
                })
            })
            .collect::<Vec<_>>();
        let mut hands = futures::future::join_all(reqs).await;

        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (hand.2 * (i + 1)) as u32)
            .sum()
    })
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

#[aoc(day7, part2, am)]
pub fn part_2_am(input: &str) -> u32 {
    WORLD.block_on(async move {
        let reqs = input
            .lines()
            .map(|x| {
                WORLD.exec_am_local(Part2 {
                    line: x.to_string(),
                })
            })
            .collect::<Vec<_>>();
        let mut hands = futures::future::join_all(reqs).await;

        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (hand.2 * (i + 1)) as u32)
            .sum()
    })
}
