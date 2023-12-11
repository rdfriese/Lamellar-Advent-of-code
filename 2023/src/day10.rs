use std::ops::DerefMut;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::WORLD;
use aoc_runner_derive::aoc;
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

//can declare this as a generator as it doesnt work with the benchmarking tool
// #[aoc_generator(day10, part1)]
fn parse_input(input: &str) -> LocalRwDarc<Vec<Vec<u8>>> {
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
pub fn part_1(_: &str) -> usize {
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
        b'S' => None,
        _ => None,
    }
}

// fn print_data(data: &[Vec<u8>]) {
//     for line in data {
//         println!("{}", String::from_utf8_lossy(line.as_slice()));
//     }
//     println!();
// }

fn find_start(data: &[Vec<u8>]) -> ((usize, usize), (usize, usize), (usize, usize), u8) {
    let mut paths = vec![];
    let mut start_char: u8 = 0b0000;
    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == b'S' as u8 {
                // top
                if i > 0 {
                    if start_valid_next(data[i - 1][j], i, j, i - 1, j) {
                        paths.push(((i, j), (i - 1, j)));
                        start_char |= 0b0001;
                    }
                }
                // bottom
                if i < data.len() {
                    if start_valid_next(data[i + 1][j], i, j, i + 1, j) {
                        paths.push(((i, j), (i + 1, j)));
                        start_char |= 0b0010;
                    }
                }
                // left
                if j > 0 {
                    if start_valid_next(data[i][j - 1], i, j, i, j - 1) {
                        paths.push(((i, j), (i, j - 1)));
                        start_char |= 0b0100;
                    }
                }
                // right
                if j < line.len() {
                    if start_valid_next(data[i][j + 1], i, j, i, j + 1) {
                        paths.push(((i, j), (i, j + 1)));
                        start_char |= 0b1000;
                    }
                }
            }
        }
    }
    (paths[0].0, paths[0].1, paths[1].1, start_char)
}
fn part1(data: &mut [Vec<u8>]) -> isize {
    let mut cnt = 0;
    let (start, mut cur, _, start_char) = find_start(data);
    let mut prev = start;
    while let Some(next) = next_tile(data[cur.0][cur.1], cur.0, cur.1, prev.0, prev.1) {
        match data[cur.0][cur.1] {
            b'|' => data[cur.0][cur.1] = b'!',
            b'-' => data[cur.0][cur.1] = b'=',
            b'L' => data[cur.0][cur.1] = b'l',
            b'J' => data[cur.0][cur.1] = b'j',
            b'7' => data[cur.0][cur.1] = b'z',
            b'F' => data[cur.0][cur.1] = b'f',
            _ => {}
        }
        prev = cur;
        cur = next;
        cnt += 1;
    }

    match start_char {
        0b0011 => data[start.0][start.1] = b'!',
        0b0101 => data[start.0][start.1] = b'j',
        0b0110 => data[start.0][start.1] = b'z',
        0b1001 => data[start.0][start.1] = b'l',
        0b1010 => data[start.0][start.1] = b'f',
        0b1100 => data[start.0][start.1] = b'=',
        _ => {}
    }
    (cnt as f32 / 2.0).ceil() as isize
}

// have each path keep track of its own path cnt, the sum and divide by two
#[AmLocalData]
struct Part1 {
    data: LocalRwDarc<Vec<Vec<u8>>>,
    prev: (usize, usize),
    cur: (usize, usize),
    cnt: Darc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() {
        let mut cur = self.cur;
        let mut prev = self.prev;
        let mut cur_val = self.data.read().await[cur.0][cur.1];
        while let Some(next) = next_tile(cur_val, cur.0, cur.1, prev.0, prev.1) {
            let mut data = self.data.write().await;
            match cur_val {
                b'|' => data[cur.0][cur.1] = b'!',
                b'-' => data[cur.0][cur.1] = b'=',
                b'L' => data[cur.0][cur.1] = b'l',
                b'J' => data[cur.0][cur.1] = b'j',
                b'7' => data[cur.0][cur.1] = b'z',
                b'F' => data[cur.0][cur.1] = b'f',
                _ => {}
            }
            prev = cur;
            cur = next;
            self.cnt.fetch_add(1, Ordering::Relaxed);
            cur_val = data[cur.0][cur.1];
        }
    }
}

//cant run bench mode because we modify the input data which
//will cause issues in subsequent runs
#[aoc(day10, part1, serial)]
pub fn part_1_serial(data: &str) -> isize {
    let data = parse_input(data);
    let mut data_guard = WORLD.block_on(data.write());
    part1(data_guard.deref_mut())
}

#[aoc(day10, part1, am)]
pub fn part_1_am(data: &str) -> usize {
    let data = parse_input(data);
    let cnt = Darc::new(&*WORLD, AtomicUsize::new(0)).unwrap();
    let mut data_guard = WORLD.block_on(data.write());
    let (start, path1, path2, start_char) = find_start(data_guard.deref_mut());
    WORLD.exec_am_local(Part1 {
        data: data.clone(),
        prev: start,
        cur: path1,
        cnt: cnt.clone(),
    });
    WORLD.exec_am_local(Part1 {
        data: data.clone(),
        prev: start,
        cur: path2,
        cnt: cnt.clone(),
    });

    match start_char {
        0b0011 => data_guard[start.0][start.1] = b'!',
        0b0101 => data_guard[start.0][start.1] = b'j',
        0b0110 => data_guard[start.0][start.1] = b'z',
        0b1001 => data_guard[start.0][start.1] = b'l',
        0b1010 => data_guard[start.0][start.1] = b'f',
        0b1100 => data_guard[start.0][start.1] = b'=',
        _ => {}
    }
    drop(data_guard);
    WORLD.wait_all();
    cnt.load(Ordering::SeqCst) / 2
}

fn vertical_advance_to_non_edge<I, W>(
    data: &[Vec<u8>],
    i: &mut usize,
    j: usize,
    inc: I,
    work_left: W,
) -> usize
where
    I: Fn(usize) -> usize,
    W: Fn(usize) -> bool,
{
    let start = data[*i][j];
    *i = inc(*i);
    if start == b'=' {
        return 1;
    }

    while work_left(*i) {
        match data[*i][j] {
            b'!' => {} //still on the "edge", could go under the catch all case but this is more explicit
            b'f' | b'l' | b'j' | b'z' => {
                if (start == b'j' && data[*i][j] == b'z')
                    || (start == b'l' && data[*i][j] == b'f')
                    || (start == b'z' && data[*i][j] == b'j')
                    || (start == b'f' && data[*i][j] == b'l')
                {
                    *i = inc(*i);
                    // an outer edge of a "curve"
                    return 0;
                } else {
                    *i = inc(*i);
                    return 1;
                }
            }
            _ => {}
        }
        *i = inc(*i);
    }
    1
}

fn horizontal_advance_to_non_edge(data: &[Vec<u8>], i: usize, j: &mut usize) -> usize {
    let start = data[i][*j];
    *j += 1;
    if start == b'!' {
        return 1;
    }

    while *j < data[i].len() {
        match data[i][*j] {
            b'=' => {} //still on the "edge", could go under the catch all case but this is more explicit
            b'f' | b'l' | b'j' | b'z' => {
                if (start == b'f' && data[i][*j] == b'z') || (start == b'l' && data[i][*j] == b'j')
                {
                    *j += 1;
                    // an outer edge of a "curve"
                    return 0;
                } else {
                    *j += 1;
                    return 1;
                }
            }
            _ => {}
        }
        *j += 1;
    }
    1
}

pub fn check_north(data: &[Vec<u8>], mut i: usize, j: usize) -> bool {
    i -= 1;
    let mut crossed_edges = 0;
    while i as isize >= 0 {
        match data[i][j] {
            b'!' | b'f' | b'l' | b'j' | b'z' | b'=' => {
                crossed_edges +=
                    vertical_advance_to_non_edge(data, &mut i, j, |x| x - 1, |x| x as isize >= 0);
            }
            _ => {
                i -= 1;
            }
        }
    }
    crossed_edges % 2 == 1
}

pub fn check_south(data: &[Vec<u8>], mut i: usize, j: usize) -> bool {
    i += 1;
    let mut crossed_edges = 0;
    let len = data.len();
    while i < len {
        match data[i][j] {
            b'!' | b'f' | b'l' | b'j' | b'z' | b'=' => {
                crossed_edges +=
                    vertical_advance_to_non_edge(data, &mut i, j, |x| x + 1, |x| x < len);
            }
            _ => {
                i += 1;
            }
        }
    }
    crossed_edges % 2 == 1
}

fn part2(data: &[Vec<u8>], i_start: usize, i_len: usize) -> usize {
    let mut cnt = 0;
    for i in i_start..i_start + i_len {
        let mut crossed_edges = 0;
        let mut tmp_cnt = 0;
        let mut j = 0;
        while j < data[i].len() {
            match data[i][j] {
                b'!' | b'f' | b'l' | b'j' | b'z' | b'=' => {
                    cnt += tmp_cnt;
                    tmp_cnt = 0;
                    crossed_edges += horizontal_advance_to_non_edge(data, i, &mut j);
                }
                _ => {
                    if crossed_edges % 2 == 1 && check_north(data, i, j) && check_south(data, i, j)
                    {
                        tmp_cnt += 1;
                    }
                    j += 1;
                }
            }
        }
        if crossed_edges % 2 == 1 {
            cnt += tmp_cnt;
        }
    }
    cnt
}

#[aoc(day10, part2, serial)]
pub fn part_2_serial(data: &str) -> usize {
    let data = parse_input(data);
    let mut data_guard = WORLD.block_on(data.write());
    part1(data_guard.deref_mut());
    let len = data_guard.len();
    part2(data_guard.deref_mut(), 0, len)
}

#[AmData]
struct Part2 {
    data: LocalRwDarc<Vec<Vec<u8>>>,
    start: usize,
    length: usize,
    cnt: Darc<AtomicUsize>,
}

#[am]
impl LamellarAm for Part2 {
    async fn exec() {
        let data = self.data.read().await;
        let cnt = part2(&data, self.start, self.length);
        self.cnt.fetch_add(cnt, Ordering::Relaxed);
    }
}

#[aoc(day10, part2, am)]
pub fn part_2_am(data: &str) -> usize {
    let data = parse_input(data);
    let cnt = Darc::new(&*WORLD, AtomicUsize::new(0)).unwrap();
    let mut data_guard = WORLD.block_on(data.write());
    let num_threads = WORLD.num_threads_per_pe();
    let num_lines_per_thread = std::cmp::max(1, data_guard.len() / num_threads); //for the test inputs
    data_guard
        .chunks(num_lines_per_thread)
        .enumerate()
        .for_each(|(i, d)| {
            WORLD.exec_am_local(Part2 {
                data: data.clone(),
                start: i * num_lines_per_thread,
                length: d.len(),
                cnt: cnt.clone(),
            });
        });
    part1(data_guard.deref_mut());
    drop(data_guard);
    WORLD.wait_all();
    cnt.load(Ordering::SeqCst)
}
