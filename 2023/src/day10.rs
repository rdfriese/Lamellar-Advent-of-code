use std::ops::DerefMut;

use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

#[aoc_generator(day10)]
fn parse_part1(input: &str) -> LocalRwDarc<Vec<Vec<u8>>> {
    LocalRwDarc::new(
        &*WORLD,
        input
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[aoc(day10, part1, A_INIT_WORLD)]
pub fn part_1(_: &LocalRwDarc<Vec<Vec<u8>>>) -> usize {
    WORLD.num_pes()
}

fn start_valid_next(next: u8, cur_i: usize, cur_j: usize, next_i: usize, next_j: usize) -> bool {
    match next {
        b'|' => {
            if cur_j == next_j && (cur_i - 1 == next_i || cur_i + 1 == next_i) {
                return true;
            } else {
                false
            }
        }
        b'-' => {
            if cur_i == next_i && (cur_j - 1 == next_j || cur_j + 1 == next_j) {
                return true;
            } else {
                false
            }
        }
        b'L' => {
            if cur_i + 1 == next_i || cur_j - 1 == next_j {
                true
            } else {
                false
            }
        }
        b'J' => {
            if cur_i + 1 == next_i || cur_j + 1 == next_j {
                true
            } else {
                false
            }
        }
        b'7' => {
            if cur_i - 1 == next_i || cur_j + 1 == next_j {
                true
            } else {
                false
            }
        }
        b'F' => {
            if cur_i - 1 == next_i || cur_j - 1 == next_j {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn next_tile(me: u8, i: usize, j: usize, prev_i: usize, prev_j: usize) -> Option<(usize, usize)> {
    // println!(
    //     "me: {}, i: {}, j: {}, prev_i: {}, prev_j: {}",
    //     me as char, i, j, prev_i, prev_j,
    // );
    match me {
        b'|' => Some((i - prev_i + i, j)), // if prev_i < i { (i + 1, j) } else { (i - 1, j) }
        b'-' => Some((i, j - prev_j + j)), // if prev_j < j { (i, j + 1) } else { (i, j - 1) }
        b'L' => {
            if prev_i < i {
                Some((i, j + 1))
            } else {
                Some((i - 1, j))
            }
        }
        b'J' => {
            if prev_i < i {
                Some((i, j - 1))
            } else {
                Some((i - 1, j))
            }
        }
        b'7' => {
            if prev_i > i {
                Some((i, j - 1))
            } else {
                Some((i + 1, j))
            }
        }
        b'F' => {
            if prev_i > i {
                Some((i, j + 1))
            } else {
                Some((i + 1, j))
            }
        }
        b'X' => None,
        _ => None,
    }
}

fn print_data(data: &[Vec<u8>]) {
    for line in data {
        println!("{}", String::from_utf8_lossy(line.as_slice()));
    }
    println!();
}
fn part1(data: &mut [Vec<u8>]) -> isize {
    let mut paths = vec![];
    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == b'S' as u8 {
                // top
                if i > 0 {
                    if start_valid_next(data[i - 1][j], i, j, i - 1, j) {
                        paths.push(((i, j), (i - 1, j)))
                    }
                }
                // bottom
                if i < data.len() {
                    if start_valid_next(data[i + 1][j], i, j, i + 1, j) {
                        paths.push(((i, j), (i + 1, j)))
                    }
                }
                // left
                if j > 0 {
                    if start_valid_next(data[i][j - 1], i, j, i, j - 1) {
                        paths.push(((i, j), (i, j - 1)))
                    }
                }
                // right
                if j < line.len() {
                    if start_valid_next(data[i][j + 1], i, j, i, j + 1) {
                        paths.push(((i, j), (i, j + 1)))
                    }
                }
            }
        }
    }
    // println!("paths: {:?}", paths);
    // print_data(data);
    // let mut cnts = [0, 0];
    // let mut done = false;
    // let mut debug_cnt = 25;
    // while !done {
    //     paths
    //         .iter_mut()
    //         .zip(cnts.iter_mut())
    //         .for_each(|((prev, cur), cnt)| {
    //             match next_tile(data[cur.0][cur.1], cur.0, cur.1, prev.0, prev.1) {
    //                 Some(x) => {
    //                     data[cur.0][cur.1] = 'X' as u8;
    //                     *prev = *cur;
    //                     *cur = x;
    //                     *cnt += 1;
    //                 }
    //                 None => done = true,
    //             }
    //         });
    //     // print_data(data);
    //     // println!("paths: {:?}", paths);
    // }
    // std::cmp::max(cnts[0], cnts[1])
    let (mut prev, mut cur) = paths[0];
    let mut cnt = 0;
    while let Some(next) = next_tile(data[cur.0][cur.1], cur.0, cur.1, prev.0, prev.1) {
        data[cur.0][cur.1] = 'X' as u8;
        prev = cur;
        cur = next;
        cnt += 1;
    }
    (cnt as f32 / 2.0).ceil() as isize
}

// have each path keep track of its own path cnt, the sum and divide by two
#[AmData]
struct Part1 {
    data: Darc<Vec<Vec<i32>>>,
    start: usize,
    length: usize,
}

// #[am]
// impl LamellarAm for Part1 {
//     async fn exec() -> isize {
//         part1(&self.data[self.start..self.start + self.length])
//     }
// }

//cant run bench mode because we modify the input data which
//will cause issues in subsequent runs
#[aoc(day10, part1, serial)]
pub fn part_1_serial(data: &LocalRwDarc<Vec<Vec<u8>>>) -> isize {
    let mut data_guard = WORLD.block_on(data.write());
    part1(data_guard.deref_mut())
}

// #[aoc(day10, part1, am)]
// pub fn part_1(data: &Darc<Vec<Vec<u8>>>) -> isize {
//     let num_threads = WORLD.num_threads_per_pe();
//     let num_lines_per_thread = std::cmp::max(1, data.len() / num_threads); //for the test inputs
//     let reqs = data.chunks(num_lines_per_thread).enumerate().map(|(i, d)| {
//         WORLD.exec_am_local(Part1 {
//             data: data.clone(),
//             start: i * num_lines_per_thread,
//             length: d.len(),
//         })
//     });
//     WORLD
//         .block_on(futures::future::join_all(reqs))
//         .iter()
//         .sum::<isize>()
// // }

// pub fn part2_sum(data: &[Vec<i32>]) -> isize {
//     data.iter()
//         .map(|d| {
//             let mut firsts = vec![d[0]];
//             let mut next = d
//                 .as_slice()
//                 .windows(2)
//                 .map(|x| x[1] - x[0])
//                 .collect::<Vec<i32>>();
//             while next.iter().any(|x| *x != 0) {
//                 firsts.push(next[0]);
//                 next = next
//                     .as_slice()
//                     .windows(2)
//                     .map(|x| x[1] - x[0])
//                     .collect::<Vec<i32>>();
//             }
//             firsts.iter().rev().fold(0, |acc, x| x - acc) as isize
//         })
//         .sum()
// }

// #[AmData]
// struct Part2 {
//     data: Darc<Vec<Vec<i32>>>,
//     start: usize,
//     length: usize,
// }

// #[am]
// impl LamellarAm for Part2 {
//     async fn exec() -> isize {
//         part2_sum(&self.data[self.start..self.start + self.length])
//     }
// }

// #[aoc(day10, part2, serial)]
// pub fn part_2_serial(data: &Darc<Vec<Vec<i32>>>) -> isize {
//     part2_sum(data)
// }

// #[aoc(day10, part2, am)]
// pub fn part_2_am(data: &Darc<Vec<Vec<i32>>>) -> isize {
//     let num_threads = WORLD.num_threads_per_pe();
//     let num_lines_per_thread = std::cmp::max(1, data.len() / num_threads); //for the test inputs
//     let reqs = data.chunks(num_lines_per_thread).enumerate().map(|(i, d)| {
//         WORLD.exec_am_local(Part2 {
//             data: data.clone(),
//             start: i * num_lines_per_thread,
//             length: d.len(),
//         })
//     });
//     WORLD
//         .block_on(futures::future::join_all(reqs))
//         .iter()
//         .sum::<isize>()
// }
