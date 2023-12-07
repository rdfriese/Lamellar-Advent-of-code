use crate::WORLD;
use aoc_runner_derive::aoc;
use itertools::Itertools;
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
    let unique = hand.iter().map(|x| *x).unique().collect::<Vec<u8>>();
    calc_score(&unique, hand)
}

fn calc_score(unique: &Vec<u8>, hand: &Vec<u8>) -> usize {
    match unique.len() {
        5 => 0, //high card
        4 => 1, //two pair
        3 => {
            // three of a kind or two pair
            let mut cnt = hand.iter().filter(|&x| *x == unique[0]).count();
            if cnt == 1 {
                cnt = hand.iter().filter(|&x| *x == unique[1]).count();
            }
            match cnt {
                1 => 3, //three of a kind
                3 => 3, //three of a kind
                _ => 2, //two pair
            }
        }
        2 => {
            // full house or four of a kind
            let cnt = hand.iter().filter(|&x| *x == unique[0]).count();
            match cnt {
                1 => 5, //four of a kind
                4 => 5, //four of a kind
                _ => 4, //full house
            }
        }
        1 => 6, //five of a kind
        _ => 0,
    }
}

fn calc_score_part2(hand: &Vec<u8>) -> usize {
    let mut joker_cnt = 0;
    let unique = hand
        .iter()
        .map(|x| {
            if *x == 1 {
                joker_cnt += 1;
            }
            *x
        })
        .unique()
        .collect::<Vec<u8>>();
    if joker_cnt == 0 {
        calc_score(&unique, hand)
    } else {
        match unique.len() {
            5 => 1, //high card + 1 joker so two pair
            4 => 3, //either a two pair and a 1 joker or 2 joker and any other card = three of a kind
            3 => {
                // either a two pair and 1 joker, or a three of a kind, 1 joker, and 1 extra, or three jokers and two other cards
                match joker_cnt {
                    1 => {
                        // full house or four of a kind
                        let cnt = match unique[0] {
                            1 => hand.iter().filter(|&x| *x == unique[1]).count(),
                            _ => hand.iter().filter(|&x| *x == unique[0]).count(),
                        };
                        match cnt {
                            2 => 4, //full house
                            _ => 5, //four of a kind
                        }
                    }
                    _ => 5, //joker_cnt == 3 --four of a kind
                }
            }
            2 => 6, // either a two pair and 3 joker or 3 of a kind and 2 joker = five of a kind
            1 => 6, //five of a kind
            _ => 0,
        }
    }
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

// #[aoc(day7, part1, am)]
// pub fn part_1_am(input: &(Vec<f64>, Vec<f64>)) -> u32 {
//     let reqs = input
//         .0
//         .iter()
//         .zip(input.1.iter())
//         .map(|(&time, &distance)| {
//             WORLD.exec_am_local(Part1 {
//                 time: time,
//                 distance: distance,
//             })
//         })
//         .collect::<Vec<_>>();
//     WORLD
//         .block_on(futures::future::join_all(reqs))
//         .iter()
//         .product::<u32>()
// }

// #[aoc(day7, part2, serial)]
// pub fn part_2_serial((time, distance): &(f64, f64)) -> u32 {
//     let roots = return_roots(*time, *distance);
//     roots.0.abs_diff(roots.1) + 1
// }

// #[aoc(day7, part2, am)]
// pub fn part_2_am((time, distance): &(f64, f64)) -> u32 {
//     WORLD.block_on(WORLD.exec_am_local(Part1 {
//         time: *time,
//         distance: *distance,
//     }))
// }
