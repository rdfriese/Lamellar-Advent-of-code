use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
//maybe we need to return a vec of valid splits

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day15, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

#[aoc(day15, part1, serial)]
pub fn part_1_serial(data: &str) -> usize {
    data.lines()
        .map(|l| {
            l.split(',')
                .map(|s| {
                    s.as_bytes()
                        .iter()
                        .fold(0, |acc, &b| ((acc + b as usize) * 17) % 256)
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[aoc(day15, part2, serial)]
pub fn part_2_serial(data: &str) -> usize {
    let mut boxes = vec![HashMap::new(); 256];
    for line in data.lines() {
        for (i, step) in line.split(',').enumerate() {
            let mut global_h = 0;
            let mut label_h: u128 = 0;
            let bytes = step.as_bytes();
            for j in 0..bytes.len() {
                match bytes[j] {
                    b'-' => {
                        boxes[global_h].remove(&label_h);
                        break;
                    }
                    b'=' => {
                        boxes[global_h]
                            .entry(label_h)
                            .and_modify(|(_, lens)| *lens = bytes[j + 1] - 48)
                            .or_insert((i, bytes[j + 1] - 48));
                        break;
                    }
                    _ => {
                        global_h = ((global_h + bytes[j] as usize) * 17) % 256;
                        label_h = (label_h << 8) | bytes[j] as u128;
                    }
                }
            }
        }
    }

    boxes
        .drain(..)
        .enumerate()
        .map(|(i, b)| {
            let mut vec = b.into_values().collect::<Vec<_>>();
            vec.sort_unstable();
            vec.iter()
                .enumerate()
                .map(|(j, (_, f))| (i + 1) * (j + 1) * *f as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[AmLocalData]
struct Part1 {
    data: Arc<Vec<Vec<u8>>>,
    start: usize,
    n: usize,
    sum: Arc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() {
        self.sum.fetch_add(
            self.data[self.start..]
                .iter()
                .step_by(self.n)
                .map(|s| s.iter().fold(0, |acc, &b| ((acc + b as usize) * 17) % 256))
                .sum::<usize>(),
            Ordering::SeqCst,
        );
    }
}

#[aoc_generator(day15, part1, am)]
fn parse_input_1_am(input: &str) -> std::sync::Arc<Vec<Vec<u8>>> {
    let mut steps = vec![];
    input.lines().for_each(|l| {
        l.split(',').for_each(|s| steps.push(s.as_bytes().to_vec()));
    });
    std::sync::Arc::new(steps)
}

#[aoc(day15, part1, am)]
pub fn part_1_am(input: &std::sync::Arc<Vec<Vec<u8>>>) -> usize {
    let num_threads = WORLD.num_threads_per_pe();
    let sum = Arc::new(AtomicUsize::new(0));

    (0..num_threads).for_each(|t| {
        WORLD.exec_am_local(Part1 {
            data: input.clone(),
            start: t,
            n: num_threads,
            sum: sum.clone(),
        });
    });
    WORLD.wait_all();
    sum.load(Ordering::SeqCst)
}

// because the order in which the same labels are processed need to be quaranteed
// we can simply parallize over the steps, instead we will group the steps by level
// and then parallelize over the label groups
// #[AmLocalData]
// struct Part2 {
//     data: Arc<Vec<Vec<u8>>>,
//     start: usize,
//     n: usize,
//     boxes: Arc<Vec<Mutex<HashMap<u128, (usize, u8)>>>>,
// }

// #[local_am]
// impl LamellarAm for Part2 {
//     async fn exec() {}
// }

// #[aoc_generator(day15, part2, am)]
// fn parse_input_2_am(input: &str) -> std::sync::Arc<Vec<Vec<u8>>> {}

// #[aoc(day15, part2, am)]
// pub fn part_2_am(input: &std::sync::Arc<Vec<Vec<u8>>>) -> usize {}
