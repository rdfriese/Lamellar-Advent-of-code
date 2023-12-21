use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};

use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;

// This is a pretty similar problem to day 10,
// but bewteen then and now, I stumbled upon the Shoelace formula
// for computing the area of a polygon

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day18, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}
#[aoc_generator(day18, part1)]
fn parse(input: &str) -> Arc<Vec<(Dir, isize)>> {
    let data = input
        .lines()
        .map(|line| {
            let mut vals = line.split_whitespace();
            let dir = vals.next().unwrap();
            let num = vals.next().unwrap().parse::<usize>().unwrap();
            let dir = match dir {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => unreachable!(),
            };
            (dir, num as isize)
        })
        .collect::<Vec<_>>();
    Arc::new(data)
}

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn calc_area(data: &[(Dir, isize)]) -> isize {
    let mut area = 0;
    let mut perimiter = 0;
    let mut previous = (0isize, 0isize);
    let mut current = previous;
    for (dir, num) in data.iter() {
        previous = current;
        current = match dir {
            Dir::Up => (current.0, current.1 - num),
            Dir::Down => (current.0, current.1 + num),
            Dir::Left => (current.0 - num, current.1),
            Dir::Right => (current.0 + num, current.1),
        };
        area += (previous.0 * current.1) - (previous.1 * current.0);
        perimiter += num;
    }
    //based on picks theorem
    area / 2 + perimiter / 2 + 1
}

#[aoc(day18, part1, serial)]
pub fn part_1_serial(data: &Arc<Vec<(Dir, isize)>>) -> isize {
    calc_area(&data)
}

#[aoc_generator(day18, part2)]
fn parse2(input: &str) -> Arc<Vec<(Dir, isize)>> {
    let data = input
        .lines()
        .map(|line| {
            let vals = line.split('#').last().unwrap();
            let num = usize::from_str_radix(&vals[0..5], 16).unwrap();
            let dir = match vals.chars().nth(5).unwrap() {
                '0' => Dir::Right,
                '1' => Dir::Down,
                '2' => Dir::Left,
                '3' => Dir::Up,
                _ => unreachable!(),
            };
            (dir, num as isize)
        })
        .collect::<Vec<_>>();
    Arc::new(data)
}

#[aoc(day18, part2, serial)]
pub fn part_2_serial(data: &Arc<Vec<(Dir, isize)>>) -> isize {
    calc_area(&data)
}
