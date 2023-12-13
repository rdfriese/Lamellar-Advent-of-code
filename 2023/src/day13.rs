use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;
//maybe we need to return a vec of valid splits

fn parse_input(input: &str) -> Vec<Vec<Vec<u8>>> {
    let mut data = Vec::new();
    let mut puzzle = Vec::new();
    for line in input.lines() {
        if line.len() == 0 {
            data.push(puzzle);
            puzzle = Vec::new();
        } else {
            puzzle.push(line.as_bytes().to_vec());
        }
    }
    data.push(puzzle);
    data
}

fn v_find_palindrome(line: &[u8]) -> Vec<usize> {
    let mut set = Vec::new();
    for i in 1..line.len() {
        if v_check_palindrome_at_index(line, i) {
            set.push(i);
        }
    }
    set
}

fn v_check_palindrome_at_index(line: &[u8], i: usize) -> bool {
    line[..i]
        .iter()
        .rev()
        .zip(line[i..].iter())
        .filter(|(a, b)| a != b)
        .count()
        == 0
}

fn h_find_palindrome(puzzle: &[Vec<u8>], col: usize) -> Vec<usize> {
    let mut set = Vec::new();
    for row in 1..puzzle.len() {
        if h_check_palindrome_at_index(puzzle, col, row) {
            set.push(row);
        }
    }
    set
}

fn h_check_palindrome_at_index(puzzle: &[Vec<u8>], col: usize, row: usize) -> bool {
    puzzle[..row]
        .iter()
        .rev()
        .zip(puzzle[row..].iter())
        .filter(|(a, b)| a[col] != b[col])
        .count()
        == 0
}

fn part1_parse_puzzle(puzzle: &[Vec<u8>]) -> usize {
    let mut set = v_find_palindrome(&puzzle[0]);
    for line in puzzle[1..].iter() {
        set = set
            .drain(..)
            .filter(|i| v_check_palindrome_at_index(line, *i))
            .collect();

        if set.len() == 0 {
            break;
        }
    }

    if set.len() == 1 {
        return set[0];
    } else {
        // check horizontal palindromes
        let mut set = h_find_palindrome(&puzzle, 0);
        for col in 1..puzzle[0].len() {
            set = set
                .drain(..)
                .filter(|i| h_check_palindrome_at_index(&puzzle, col, *i))
                .collect();

            if set.len() == 0 {
                break;
            }
        }
        if set.len() == 1 {
            return set[0] * 100;
        }
    }
    0
}

#[aoc_generator(day13, part1, serial)]
fn parse_serial1(input: &str) -> Vec<Vec<Vec<u8>>> {
    parse_input(input)
}

#[aoc(day13, part1, serial)]
pub fn part_1_serial(data: &Vec<Vec<Vec<u8>>>) -> usize {
    let mut cnt = 0;
    for puzzle in data {
        cnt += part1_parse_puzzle(&puzzle);
    }
    cnt
}

pub fn part2_parse_puzzle(puzzle: &[Vec<u8>]) -> usize {
    let mut set_cnt = HashMap::new();

    for line in puzzle {
        for i in v_find_palindrome(&line) {
            *set_cnt.entry(i).or_insert(0) += 1;
        }
    }
    for (&k, &v) in set_cnt.iter() {
        if v == puzzle.len() - 1 {
            return k;
        }
    }
    // check horizontal palindromes

    let mut set_cnt = HashMap::new();
    for col in 0..puzzle[0].len() {
        for i in h_find_palindrome(&puzzle, col) {
            *set_cnt.entry(i).or_insert(0) += 1;
        }
    }
    for (&k, &v) in set_cnt.iter() {
        if v == puzzle[0].len() - 1 {
            return k * 100;
        }
    }
    0
}

#[aoc_generator(day13, part2, serial)]
fn parse_serial2(input: &str) -> Vec<Vec<Vec<u8>>> {
    parse_input(input)
}

#[aoc(day13, part2, serial)]
pub fn part_2_serial(data: &Vec<Vec<Vec<u8>>>) -> usize {
    let mut cnt = 0;
    for puzzle in data {
        cnt += part2_parse_puzzle(&puzzle);
    }
    cnt
}

#[aoc_generator(day13, part1, am)]
fn parse_am1(input: &str) -> Darc<Vec<Vec<Vec<u8>>>> {
    Darc::new(&*WORLD, parse_input(input)).unwrap()
}
#[AmLocalData]
struct Part1 {
    data: Darc<Vec<Vec<Vec<u8>>>>,
    start: usize,
    n: usize,
    sum: Darc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() {
        let mut sum = 0;
        for puzzle in self.data[self.start..self.data.len()]
            .iter()
            .step_by(self.n)
        {
            sum += part1_parse_puzzle(&puzzle);
        }
        self.sum.fetch_add(sum, Ordering::SeqCst);
    }
}

#[aoc(day13, part1, am)]
pub fn part_1_am(data: &Darc<Vec<Vec<Vec<u8>>>>) -> usize {
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

#[aoc_generator(day13, part2, am)]
fn parse_am2(input: &str) -> Darc<Vec<Vec<Vec<u8>>>> {
    Darc::new(&*WORLD, parse_input(input)).unwrap()
}

#[AmLocalData]
struct Part2 {
    data: Darc<Vec<Vec<Vec<u8>>>>,
    start: usize,
    n: usize,
    sum: Darc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec() {
        let mut sum = 0;
        for puzzle in self.data[self.start..self.data.len()]
            .iter()
            .step_by(self.n)
        {
            sum += part2_parse_puzzle(&puzzle);
        }
        self.sum.fetch_add(sum, Ordering::SeqCst);
    }
}

#[aoc(day13, part2, am)]
pub fn part_2_am(data: &Darc<Vec<Vec<Vec<u8>>>>) -> usize {
    let cnt = Darc::new(&*WORLD, AtomicUsize::new(0)).unwrap();
    let num_threads = WORLD.num_threads_per_pe();
    for t in 0..num_threads {
        WORLD.exec_am_local(Part2 {
            data: data.clone(),
            start: t,
            n: num_threads,
            sum: cnt.clone(),
        });
    }
    WORLD.wait_all();
    cnt.load(Ordering::SeqCst)
}
