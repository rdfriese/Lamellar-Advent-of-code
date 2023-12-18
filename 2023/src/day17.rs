use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
//maybe we need to return a vec of valid splits

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day17, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}
// #[aoc_generator(day17)]
fn parse(input: &str) -> Vec<Vec<Block>> {
    let mut data = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, e)| Block {
                    pos: (i as u32, j as u32),
                    cost: u32::MAX,
                    h_cost: u32::MAX,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // for row in &mut data {
    //     for block in row {
    //         block.h_cost = manhattan_distance(block.pos.0, block.pos.1, data.len() as u32 - 1, data[0].len() as u32 - 1);
    //     }
    // }
    // Arc::new((data))
    data
}

fn manhattan_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> usize {
    (x1 as i32 - x2 as i32).abs() as usize + (y1 as i32 - y2 as i32).abs() as usize
}

struct Block {
    pos: (u32, u32),
    cost: u32,   //actual cost
    h_cost: u32, //heuristic cost
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_cost = self.cost + self.h_cost;
        let other_cost = other.cost + other.h_cost;
        other.h_cost.cmp(&self.h_cost).then_with(|| {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.pos.cmp(&other.pos))
        }) //reverse order makes this a min head
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day17, part1, serial)]
pub fn part_1_serial(input: &str) -> usize {
    let mut data = parse(input);
    let mut cur = data[0][0];
    let end = (data.0.len() - 1, data.0[0].len() - 1);
    heap.push(cur);

    while let Some(cur) = heap.pop() {
        if cur.pos == end {
            return cur.cost + data[end.pos.0 as usize][end.pos.1 as usize].cost;
        }
    }
}
