use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::WORLD;
use aoc_runner_derive::aoc;
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

fn parse_input(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut data = Vec::new();
    for line in input.lines() {
        let mut line_split = line.split_ascii_whitespace();
        let first = line_split.next().unwrap();
        let second = line_split
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u8>>();
        data.push((first.as_bytes().to_vec(), second));
    }
    data
}

fn parse_input2(input: &str, unfold_factor: usize) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut data = Vec::new();
    for line in input.lines() {
        let mut line_split = line.split_ascii_whitespace();
        let mut first = line_split.next().unwrap().as_bytes().to_vec();
        first.push(b'?');
        let mut first = first.repeat(unfold_factor);
        first.pop();
        let second = line_split
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u8>>();
        let second = second.repeat(unfold_factor);

        data.push((first, second));
    }
    data
}

#[aoc(day12, part1, A_INIT_WORLD)]
pub fn part_1(_: &str) -> usize {
    WORLD.num_pes()
}

fn cnt_configurations(line: &[u8], ranges: &[u8], cnt: &mut usize) {
    let ranges_len = ranges.iter().map(|x| *x as usize).sum::<usize>() as usize;
    for (i, w) in line
        .windows(ranges[0] as usize)
        .enumerate()
        .take_while(|(i, _)| i + ranges_len <= line.len())
    {
        if w.iter().all(|x| *x == b'#' || *x == b'?') {
            if ranges.len() == 1 {
                if !line[i + ranges[0] as usize..].iter().any(|x| *x == b'#') {
                    *cnt += 1;
                }
            } else if line[i + ranges[0] as usize] != b'#' {
                cnt_configurations(&line[i + 1 + ranges[0] as usize..], &ranges[1..], cnt);
            }
            if !w.iter().any(|x| *x == b'?') {
                return;
            }
        }
        if w[0] == b'#' {
            return;
        }
    }
}

fn cnt_configurations2<'a>(
    line: &'a [u8],
    ranges: &'a [u8],
    mut memory: HashMap<(usize, usize), usize>,
) -> (usize, HashMap<(usize, usize), usize>) {
    let ranges_len = ranges.iter().map(|x| *x as usize).sum::<usize>() as usize;
    let mut tmp_cnt = 0;
    if let Some(c) = memory.get(&(line.len(), ranges.len())) {
        tmp_cnt = *c;
    } else {
        for (i, w) in line.windows(ranges[0] as usize).enumerate() {
            if i + ranges_len > line.len() {
                break;
            }
            if w.iter().all(|x| *x == b'#' || *x == b'?') {
                if ranges.len() == 1 {
                    if !line[i + ranges[0] as usize..].iter().any(|x| *x == b'#') {
                        tmp_cnt += 1;
                        *memory.entry((line.len(), ranges.len())).or_insert(0) += 1;
                    }
                } else if line[i + ranges[0] as usize] != b'#' {
                    let (c, m) = cnt_configurations2(
                        &line[i + 1 + ranges[0] as usize..],
                        &ranges[1..],
                        memory,
                    );
                    memory = m;
                    tmp_cnt += c;
                    *memory.entry((line.len(), ranges.len())).or_insert(0) += c;
                }
                if !w.iter().any(|x| *x == b'?') {
                    return (tmp_cnt, memory);
                }
            }
            if w[0] == b'#' {
                return (tmp_cnt, memory);
            }
        }
    }
    (tmp_cnt, memory)
}

#[aoc(day12, part1, serial)]
pub fn part_1_serial(data: &str) -> usize {
    let data = parse_input(data);
    let mut cnt = 0;
    for line in data {
        cnt_configurations(&line.0, &line.1, &mut cnt);
    }
    cnt
}

#[aoc(day12, part2, serial)]
pub fn part_2_serial(data: &str) -> usize {
    let data = parse_input2(data, 5);
    let mut cnt = 0;
    for line in data {
        let (tcnt, _) = cnt_configurations2(&line.0, &line.1, HashMap::new());
        cnt += tcnt;
    }
    cnt
}

#[AmLocalData]
struct Part1 {
    data: Darc<Vec<(Vec<u8>, Vec<u8>)>>,
    start: usize,
    n: usize,
    sum: Darc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() {
        let mut sum = 0;
        for line in self.data[self.start..self.data.len()]
            .iter()
            .step_by(self.n)
        {
            cnt_configurations(&line.0, &line.1, &mut sum);
        }
        self.sum.fetch_add(sum, Ordering::SeqCst);
    }
}

#[aoc(day12, part1, am)]
pub fn part_1_am(data: &str) -> usize {
    let data = Darc::new(&*WORLD, parse_input(data)).unwrap();
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

#[AmLocalData]
struct Part2 {
    data: Darc<Vec<(Vec<u8>, Vec<u8>)>>,
    start: usize,
    n: usize,
    sum: Darc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec() {
        let mut sum = 0;
        for line in self.data[self.start..self.data.len()]
            .iter()
            .step_by(self.n)
        {
            let (tcnt, _) = cnt_configurations2(&line.0, &line.1, HashMap::new());
            sum += tcnt;
        }
        self.sum.fetch_add(sum, Ordering::SeqCst);
    }
}

#[aoc(day12, part2, am)]
pub fn part_2_am(data: &str) -> usize {
    let data = Darc::new(&*WORLD, parse_input2(data, 5)).unwrap();
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
