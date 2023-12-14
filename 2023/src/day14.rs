use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::sync::Arc;

use crate::WORLD;
use aoc_runner_derive::aoc;
use lamellar::active_messaging::prelude::*;
//maybe we need to return a vec of valid splits

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day14, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut data = Vec::new();
    for line in input.lines() {
        data.push(line.as_bytes().to_vec());
    }
    data
}

#[aoc(day14, part1, serial)]
pub fn part_1_serial(data: &str) -> usize {
    let mut data = parse_input(data);
    let mut sum = 0;
    for j in 0..data[0].len() {
        let mut next_row = 0;
        for i in 0..data.len() {
            if data[i][j] == b'#' {
                next_row = i + 1;
            } else if data[i][j] == b'O' {
                data[i][j] = b'.';
                data[next_row][j] = b'O';

                sum += data.len() - next_row;
                next_row += 1;
            }
        }
    }
    sum
}

fn print_data(data: &[Vec<u8>]) {
    for line in data {
        println!("{:?}", std::str::from_utf8(&line));
    }
    println!("");
}

fn tilt_north(data: &mut [Vec<u8>]) -> usize {
    let mut sum = 0;
    for j in 0..data[0].len() {
        let mut next_row = 0;
        for i in 0..data.len() {
            if data[i][j] == b'#' {
                next_row = i + 1;
            } else if data[i][j] == b'O' {
                data[i][j] = b'.';
                data[next_row][j] = b'O';

                sum += data.len() - next_row;
                next_row += 1;
            }
        }
    }
    sum
}

fn tilt_south(data: &mut [Vec<u8>]) -> usize {
    let mut sum = 0;
    for j in 0..data[0].len() {
        let mut next_row = data[0].len() - 1;
        for i in (0..data.len()).rev() {
            if data[i][j] == b'#' {
                next_row = i - 1;
            } else if data[i][j] == b'O' {
                data[i][j] = b'.';
                data[next_row][j] = b'O';

                sum += data.len() - next_row;
                next_row -= 1;
            }
        }
    }
    sum
}

fn tilt_west(data: &mut [Vec<u8>]) -> usize {
    let mut sum = 0;
    for i in 0..data.len() {
        let mut next_col = 0;
        for j in 0..data[0].len() {
            if data[i][j] == b'#' {
                next_col = j + 1;
            } else if data[i][j] == b'O' {
                data[i][j] = b'.';
                data[i][next_col] = b'O';

                sum += data[0].len() - i;
                next_col += 1;
            }
        }
    }
    sum
}

fn tilt_east(data: &mut [Vec<u8>]) -> usize {
    let mut sum = 0;
    for i in 0..data.len() {
        let mut next_col = data[0].len() - 1;
        for j in (0..data[0].len()).rev() {
            if data[i][j] == b'#' {
                next_col = j - 1;
            } else if data[i][j] == b'O' {
                data[i][j] = b'.';
                data[i][next_col] = b'O';

                sum += data[0].len() - i;
                next_col -= 1;
            }
        }
    }
    sum
}

#[aoc(day14, part2, serial)]
pub fn part_2_serial(data: &str) -> usize {
    let mut data = parse_input(data);
    let mut memory = HashMap::new();
    for c in 1..=1000000000 {
        let cur = (
            tilt_north(&mut data),
            tilt_west(&mut data),
            tilt_south(&mut data),
            tilt_east(&mut data),
        );
        let prev = memory.entry(cur).or_insert(c);
        if c != *prev {
            if (1000000000 - c) % (c - *prev) == 0 {
                return cur.3;
            }
        }
    }
    0 // assume we will never actually reach this point
}

#[AmLocalData]
struct TiltNorthAm {
    data: Arc<Vec<Vec<AtomicU8>>>,
    col: usize,
    n: usize,
    sum: Arc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for TiltNorthAm {
    async fn exec() {
        // let mut sum = 0;
        let mut sum = 0;
        for j in (self.col..self.data[0].len()).step_by(self.n) {
            let mut next_row = 0;
            for i in 0..self.data.len() {
                let cur = self.data[i][j].load(Ordering::SeqCst);
                if cur == b'#' {
                    next_row = i + 1;
                } else if cur == b'O' {
                    self.data[i][j].store(b'.', Ordering::SeqCst);
                    self.data[next_row][j].store(b'O', Ordering::SeqCst);
                    sum += self.data.len() - next_row;
                    next_row += 1;
                }
            }
        }
        self.sum.fetch_add(sum, Ordering::SeqCst);
    }
}

fn parse_input_am(input: &str) -> Arc<Vec<Vec<AtomicU8>>> {
    let mut data = Vec::new();
    for line in input.lines() {
        data.push(line.as_bytes().iter().map(|x| AtomicU8::new(*x)).collect());
    }
    Arc::new(data)
}

#[aoc(day14, part1, am)]
pub fn part_1_am(data: &str) -> usize {
    let cnt = Arc::new(AtomicUsize::new(0));
    let data = parse_input_am(data);
    let num_threads = WORLD.num_threads_per_pe();
    for t in 0..num_threads {
        WORLD.exec_am_local(TiltNorthAm {
            data: data.clone(),
            col: t,
            n: num_threads,
            sum: cnt.clone(),
        });
    }
    WORLD.wait_all();
    cnt.load(Ordering::SeqCst)
}

#[AmLocalData]
struct TiltSouthAm {
    data: Arc<Vec<Vec<AtomicU8>>>,
    col: usize,
    n: usize,
    sum: Arc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for TiltSouthAm {
    async fn exec() {
        // let mut sum = 0;
        let mut sum = 0;
        for j in (self.col..self.data[0].len()).step_by(self.n) {
            let mut next_row = self.data.len() - 1;
            for i in (0..self.data.len()).rev() {
                let cur = self.data[i][j].load(Ordering::SeqCst);
                if cur == b'#' {
                    next_row = i - 1;
                } else if cur == b'O' {
                    self.data[i][j].store(b'.', Ordering::SeqCst);
                    self.data[next_row][j].store(b'O', Ordering::SeqCst);
                    sum += self.data.len() - next_row;
                    next_row -= 1;
                }
            }
        }
        self.sum.fetch_add(sum, Ordering::SeqCst);
    }
}

#[AmLocalData]
struct TiltWestAm {
    data: Arc<Vec<Vec<AtomicU8>>>,
    row: usize,
    n: usize,
    sum: Arc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for TiltWestAm {
    async fn exec() {
        // let mut sum = 0;
        let mut sum = 0;
        for i in (self.row..self.data.len()).step_by(self.n) {
            let mut next_col = 0;
            for j in 0..self.data[0].len() {
                let cur = self.data[i][j].load(Ordering::SeqCst);
                if cur == b'#' {
                    next_col = j + 1;
                } else if cur == b'O' {
                    self.data[i][j].store(b'.', Ordering::SeqCst);
                    self.data[i][next_col].store(b'O', Ordering::SeqCst);
                    sum += self.data.len() - i;
                    next_col += 1;
                }
            }
        }
        self.sum.fetch_add(sum, Ordering::SeqCst);
    }
}

#[AmLocalData]
struct TiltEastAm {
    data: Arc<Vec<Vec<AtomicU8>>>,
    row: usize,
    n: usize,
    sum: Arc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for TiltEastAm {
    async fn exec() {
        // let mut sum = 0;
        let mut sum = 0;
        for i in (self.row..self.data.len()).step_by(self.n) {
            let mut next_col = self.data[0].len() - 1;
            for j in (0..self.data[0].len()).rev() {
                let cur = self.data[i][j].load(Ordering::SeqCst);
                if cur == b'#' {
                    next_col = j - 1;
                } else if cur == b'O' {
                    self.data[i][j].store(b'.', Ordering::SeqCst);
                    self.data[i][next_col].store(b'O', Ordering::SeqCst);
                    sum += self.data.len() - i;
                    next_col -= 1;
                }
            }
        }
        self.sum.fetch_add(sum, Ordering::SeqCst);
    }
}

#[aoc(day14, part2, am)]
pub fn part_2_am(data: &str) -> usize {
    let cnt = Arc::new(AtomicUsize::new(0));
    let data = parse_input_am(data);
    let num_threads = WORLD.num_threads_per_pe();
    WORLD.block_on(async move {
        let mut memory = HashMap::new();
        for c in 1..=1000000000 {
            let mut cur = (0, 0, 0, 0);
            for t in 0..num_threads {
                WORLD.exec_am_local(TiltNorthAm {
                    data: data.clone(),
                    col: t,
                    n: num_threads,
                    sum: cnt.clone(),
                });
            }
            WORLD.wait_all();
            cur.0 = cnt.swap(0, Ordering::SeqCst);
            for t in 0..num_threads {
                WORLD.exec_am_local(TiltWestAm {
                    data: data.clone(),
                    row: t,
                    n: num_threads,
                    sum: cnt.clone(),
                });
            }
            WORLD.wait_all();
            cur.1 = cnt.swap(0, Ordering::SeqCst);
            for t in 0..num_threads {
                WORLD.exec_am_local(TiltSouthAm {
                    data: data.clone(),
                    col: t,
                    n: num_threads,
                    sum: cnt.clone(),
                });
            }
            WORLD.wait_all();
            cur.2 = cnt.swap(0, Ordering::SeqCst);
            for t in 0..num_threads {
                WORLD.exec_am_local(TiltEastAm {
                    data: data.clone(),
                    row: t,
                    n: num_threads,
                    sum: cnt.clone(),
                });
            }
            WORLD.wait_all();
            cur.3 = cnt.swap(0, Ordering::SeqCst);
            let prev = memory.entry(cur).or_insert(c);
            if c != *prev {
                if (1000000000 - c) % (c - *prev) == 0 {
                    return cur.3;
                }
            }
        }
        0 //assume never get here
    })
}
