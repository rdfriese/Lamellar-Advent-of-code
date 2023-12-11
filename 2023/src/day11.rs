use std::sync::atomic::{AtomicUsize, Ordering};

use crate::WORLD;
use aoc_runner_derive::aoc;
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

fn parse_input(input: &str, expansion_factor: usize) -> Vec<(u32, u32)> {
    //we'll just create a coordinate list, CSR is probably overkill
    let mut coords = Vec::new();

    let mut lines = input.lines().enumerate().peekable();
    let mut empty_cols = vec![true; lines.peek().unwrap().1.len()];
    let mut extra_rows = 0;
    for (j, line) in lines {
        let row = j + extra_rows;
        let line_bytes = line.as_bytes();
        let mut empty_row = true;
        for (col, b) in line_bytes.iter().enumerate() {
            if *b == b'#' {
                coords.push((row as u32, col as u32));
                empty_cols[col] = false;
                empty_row = false;
            }
        }
        if empty_row {
            extra_rows += expansion_factor - 1;
        }
    }
    let empty_cols = empty_cols
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| if x { Some(i) } else { None })
        .collect::<Vec<_>>();
    for entry in coords.iter_mut() {
        let mut col = 0;
        let mut extra_cols = 0;
        while col < empty_cols.len() && entry.1 > empty_cols[col] as u32 {
            col += 1;
            extra_cols += expansion_factor - 1;
        }
        entry.1 += extra_cols as u32;
    }
    // println!("empty cols {:?}", empty_cols);
    // println!("coords {:?}", coords);
    coords
}

fn manhattan_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> usize {
    (x1 as i32 - x2 as i32).abs() as usize + (y1 as i32 - y2 as i32).abs() as usize
}

#[aoc(day11, part1, A_INIT_WORLD)]
pub fn part_1(_: &str) -> usize {
    WORLD.num_pes()
}

#[AmLocalData]
struct Part1 {
    data: Darc<Vec<(u32, u32)>>,
    start: usize,
    n: usize,
    sum: Darc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() {
        let mut sum = 0;
        for i in (self.start..self.data.len()).step_by(self.n) {
            for j in i + 1..self.data.len() {
                let temp = manhattan_distance(
                    self.data[i].0,
                    self.data[i].1,
                    self.data[j].0,
                    self.data[j].1,
                );
                sum += temp;
            }
        }
        self.sum.fetch_add(sum, Ordering::SeqCst);
    }
}
#[aoc(day11, part1, serial)]
pub fn part_1_serial(data: &str) -> usize {
    let data = parse_input(data, 2);
    let mut sum = 0;
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            let temp = manhattan_distance(data[i].0, data[i].1, data[j].0, data[j].1);
            sum += temp;
        }
    }
    sum
}

#[aoc(day11, part2, serial)]
pub fn part_2_serial(data: &str) -> usize {
    let data = parse_input(data, 1000000);
    let mut sum = 0;
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            let temp = manhattan_distance(data[i].0, data[i].1, data[j].0, data[j].1);
            sum += temp;
        }
    }
    sum
}

#[aoc(day11, part1, am)]
pub fn part_1_am(data: &str) -> usize {
    let data = Darc::new(&*WORLD, parse_input(data, 2)).unwrap();
    let cnt = Darc::new(&*WORLD, AtomicUsize::new(0)).unwrap();
    let num_threads = WORLD.num_threads_per_pe();
    for t in 0..num_threads {
        WORLD.exec_am_local(Part1 {
            data: data.clone(),
            start: t,
            n: num_threads,
            sum: cnt.clone(),
        });
    }
    WORLD.wait_all();
    cnt.load(Ordering::SeqCst)
}

#[aoc(day11, part2, am)]
pub fn part_2_am(data: &str) -> usize {
    let data = Darc::new(&*WORLD, parse_input(data, 1000000)).unwrap();
    let cnt = Darc::new(&*WORLD, AtomicUsize::new(0)).unwrap();
    let num_threads = WORLD.num_threads_per_pe();
    for t in 0..num_threads {
        WORLD.exec_am_local(Part1 {
            data: data.clone(),
            start: t,
            n: num_threads,
            sum: cnt.clone(),
        });
    }
    WORLD.wait_all();
    cnt.load(Ordering::SeqCst)
}
