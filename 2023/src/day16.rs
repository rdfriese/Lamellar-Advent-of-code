use std::collections::HashSet;
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
#[aoc(day16, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}
#[aoc_generator(day16)]
fn parse(input: &str) -> Arc<(Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>)> {
    let mut row_non_zeros = vec![];
    let mut col_non_zeros = vec![];
    let mut lines = input.lines().enumerate();

    //process first row to get number of columns
    row_non_zeros.push(vec![]);
    let (row, line) = lines.next().unwrap();
    for (col, e) in line.as_bytes().iter().enumerate() {
        col_non_zeros.push(vec![]);
        if *e != b'.' {
            row_non_zeros[row].push((*e, col));
            col_non_zeros[col].push((*e, row));
        }
    }

    for (row, line) in lines {
        row_non_zeros.push(vec![]);
        for (col, e) in line.as_bytes().iter().enumerate() {
            if *e != b'.' {
                row_non_zeros[row].push((*e, col));
                col_non_zeros[col].push((*e, row));
            }
        }
    }
    Arc::new((row_non_zeros, col_non_zeros))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up((usize, usize)),
    Down((usize, usize)),
    Left((usize, usize)),
    Right((usize, usize)),
}

impl Dir {
    fn next(
        &self,
        energized: &mut HashSet<(usize, usize)>,
        paths: &mut HashSet<Dir>,
        row_non_zeros: &Vec<Vec<(u8, usize)>>,
        col_non_zeros: &Vec<Vec<(u8, usize)>>,
        num_rows: usize,
        num_cols: usize,
        dir_vec: &mut Vec<Dir>,
    ) -> Option<Self> {
        if paths.contains(self) {
            return None;
        }
        paths.insert(*self);
        let mut next = None;
        match self {
            Dir::Up((i, j)) => {
                energized.insert((*i, *j));
                let nz = match col_non_zeros[*j].binary_search_by(|(_, row)| row.cmp(i)) {
                    Ok(nz) => nz,
                    Err(nz) => {
                        if nz > 0 {
                            nz - 1
                        } else {
                            0
                        }
                    }
                };
                if nz < col_non_zeros[*j].len() {
                    let (e, row) = col_non_zeros[*j][nz];
                    if row <= *i {
                        for ii in row..*i {
                            energized.insert((ii, *j));
                        }
                        match e {
                            b'|' => {
                                if row > 0 {
                                    next = Some(Dir::Up((row - 1, *j)));
                                }
                            }
                            b'-' => {
                                if *j > 0 {
                                    dir_vec.push(Dir::Left((row, *j - 1)));
                                }
                                if *j < num_cols - 1 {
                                    next = Some(Dir::Right((row, *j + 1)));
                                }
                            }
                            b'/' => {
                                if *j < num_cols - 1 {
                                    next = Some(Dir::Right((row, *j + 1)));
                                }
                            }
                            b'\\' => {
                                if *j > 0 {
                                    next = Some(Dir::Left((row, *j - 1)));
                                }
                            }
                            _ => {}
                        }
                    } else {
                        for ii in 0..*i {
                            energized.insert((ii, *j));
                        }
                    }
                }
            }
            Dir::Down((i, j)) => {
                energized.insert((*i, *j));
                let nz = match col_non_zeros[*j].binary_search_by(|(_, row)| row.cmp(i)) {
                    Ok(nz) => nz,
                    Err(nz) => nz,
                };
                if nz < col_non_zeros[*j].len() {
                    let (e, row) = col_non_zeros[*j][nz];
                    for ii in *i..=row {
                        energized.insert((ii, *j));
                    }
                    match e {
                        b'|' => {
                            if row < num_rows - 1 {
                                next = Some(Dir::Down((row + 1, *j)));
                            }
                        }
                        b'-' => {
                            if *j > 0 {
                                dir_vec.push(Dir::Left((row, *j - 1)));
                            }
                            if *j < num_cols - 1 {
                                next = Some(Dir::Right((row, *j + 1)));
                            }
                        }
                        b'\\' => {
                            if *j < num_cols - 1 {
                                next = Some(Dir::Right((row, *j + 1)));
                            }
                        }
                        b'/' => {
                            if *j > 0 {
                                next = Some(Dir::Left((row, *j - 1)));
                            }
                        }
                        _ => {}
                    }
                } else {
                    for ii in *i..num_rows {
                        energized.insert((ii, *j));
                    }
                }
            }
            Dir::Left((i, j)) => {
                energized.insert((*i, *j));
                let nz = match row_non_zeros[*i].binary_search_by(|(_, col)| col.cmp(j)) {
                    Ok(nz) => nz,
                    Err(nz) => {
                        if nz > 0 {
                            nz - 1
                        } else {
                            0
                        }
                    }
                };
                if nz < row_non_zeros[*i].len() {
                    let (e, col) = row_non_zeros[*i][nz];
                    if col <= *j {
                        for jj in col..*j {
                            energized.insert((*i, jj));
                        }
                        match e {
                            b'|' => {
                                if *i > 0 {
                                    dir_vec.push(Dir::Up((*i - 1, col)));
                                }
                                if *i < num_rows - 1 {
                                    next = Some(Dir::Down((*i + 1, col)));
                                }
                            }
                            b'-' => {
                                if col > 0 {
                                    next = Some(Dir::Left((*i, col - 1)));
                                }
                            }
                            b'/' => {
                                if *i < num_rows - 1 {
                                    next = Some(Dir::Down((*i + 1, col)));
                                }
                            }
                            b'\\' => {
                                if *i > 0 {
                                    next = Some(Dir::Up((*i - 1, col)));
                                }
                            }
                            _ => {}
                        }
                    } else {
                        for jj in 0..*j {
                            energized.insert((*i, jj));
                        }
                    }
                }
            }
            Dir::Right((i, j)) => {
                energized.insert((*i, *j));
                let nz = match row_non_zeros[*i].binary_search_by(|(_, col)| col.cmp(j)) {
                    Ok(nz) => nz,
                    Err(nz) => nz,
                };
                if nz < row_non_zeros[*i].len() {
                    let (e, col) = row_non_zeros[*i][nz];
                    for jj in *j..=col {
                        energized.insert((*i, jj));
                    }
                    match e {
                        b'|' => {
                            if *i > 0 {
                                dir_vec.push(Dir::Up((*i - 1, col)))
                            }
                            if *i < num_rows - 1 {
                                next = Some(Dir::Down((*i + 1, col)))
                            }
                        }
                        b'-' => {
                            if col < num_cols - 1 {
                                next = Some(Dir::Right((*i, col + 1)))
                            }
                        }
                        b'\\' => {
                            if *i < num_rows - 1 {
                                next = Some(Dir::Down((*i + 1, col)))
                            }
                        }
                        b'/' => {
                            if *i > 0 {
                                next = Some(Dir::Up((*i - 1, col)))
                            }
                        }
                        _ => {}
                    }
                } else {
                    for jj in *j..num_cols {
                        energized.insert((*i, jj));
                    }
                }
            }
        }
        next
    }
}

fn get_energized_from_start(
    dir: Dir,
    row_non_zeros: &Vec<Vec<(u8, usize)>>,
    col_non_zeros: &Vec<Vec<(u8, usize)>>,
    num_rows: usize,
    num_cols: usize,
) -> usize {
    let mut energized = HashSet::new();
    let mut paths = HashSet::new();

    let mut dir_vec = vec![dir];
    while let Some(mut cur) = dir_vec.pop() {
        while let Some(next) = cur.next(
            &mut energized,
            &mut paths,
            row_non_zeros,
            col_non_zeros,
            num_rows,
            num_cols,
            &mut dir_vec,
        ) {
            if cur == next {
                break;
            }
            cur = next;
        }
    }
    energized.len()
}

#[aoc(day16, part1, serial)]
pub fn part_1_serial(data: &Arc<(Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>)>) -> usize {
    let row_non_zeros = &data.0;
    let col_non_zeros = &data.1;
    get_energized_from_start(
        Dir::Right((0, 0)),
        row_non_zeros,
        col_non_zeros,
        data.0.len(),
        data.1.len(),
    )
}

#[aoc(day16, part2, serial)]
pub fn part_2_serial(data: &Arc<(Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>)>) -> usize {
    let row_non_zeros = &data.0;
    let col_non_zeros = &data.1;
    let num_rows = data.0.len();
    let num_cols = data.1.len();

    let mut max = 0;
    for col in 0..num_cols {
        let energized = get_energized_from_start(
            Dir::Down((0, col)),
            row_non_zeros,
            col_non_zeros,
            num_rows,
            num_cols,
        );
        if energized > max {
            max = energized;
        }
        let energized = get_energized_from_start(
            Dir::Up((num_rows - 1, col)),
            row_non_zeros,
            col_non_zeros,
            num_rows,
            num_cols,
        );
        if energized > max {
            max = energized;
        }
    }
    for row in 0..num_rows {
        let energized = get_energized_from_start(
            Dir::Right((row, 0)),
            row_non_zeros,
            col_non_zeros,
            num_rows,
            num_cols,
        );
        if energized > max {
            max = energized;
        }
        let energized = get_energized_from_start(
            Dir::Left((row, num_cols - 1)),
            row_non_zeros,
            col_non_zeros,
            num_rows,
            num_cols,
        );
        if energized > max {
            max = energized;
        }
    }
    max
}

#[AmLocalData]
struct Part1 {
    data: Arc<(Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>)>,
    energized: Arc<Mutex<HashSet<(usize, usize)>>>,
    num_rows: usize,
    num_cols: usize,
    paths: HashSet<Dir>,
    start_dir: Dir,
    cnt: Arc<AtomicUsize>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() {
        let row_non_zeros = &self.data.0;
        let col_non_zeros = &self.data.1;
        let mut energized = HashSet::new();
        let mut paths = self.paths.clone();
        let mut dir_vec = vec![self.start_dir];
        while let Some(mut cur) = dir_vec.pop() {
            while let Some(next) = cur.next(
                &mut energized,
                &mut paths,
                row_non_zeros,
                col_non_zeros,
                self.num_rows,
                self.num_cols,
                &mut dir_vec,
            ) {
                if cur == next {
                    break;
                }
                cur = next;
                if self.cnt.load(Ordering::SeqCst) < lamellar::world.num_threads_per_pe() {
                    for dir in dir_vec.drain(..) {
                        self.cnt.fetch_add(1, Ordering::SeqCst);
                        lamellar::world.exec_am_local(Part1 {
                            data: self.data.clone(),
                            energized: self.energized.clone(),
                            num_rows: self.num_rows,
                            num_cols: self.num_cols,
                            paths: paths.clone(),
                            start_dir: dir,
                            cnt: self.cnt.clone(),
                        });
                    }
                }
            }
        }
        self.energized.lock().unwrap().extend(energized);
    }
}

#[aoc(day16, part1, am)]
pub fn part_1_am(input: &Arc<(Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>)>) -> usize {
    let energized = Arc::new(Mutex::new(HashSet::new()));
    let cnt = Arc::new(AtomicUsize::new(0));
    WORLD.exec_am_local(Part1 {
        data: input.clone(),
        energized: energized.clone(),
        num_rows: input.0.len(),
        num_cols: input.1.len(),
        paths: HashSet::new(),
        start_dir: Dir::Right((0, 0)),
        cnt: cnt.clone(),
    });
    WORLD.wait_all();
    let len = energized.lock().unwrap().len();
    len
}

#[AmLocalData]
struct Part2 {
    data: Arc<(Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>)>,
    num_rows: usize,
    num_cols: usize,
    paths: HashSet<Dir>,
    start_dir: Dir,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec() -> usize {
        let row_non_zeros = &self.data.0;
        let col_non_zeros = &self.data.1;
        let mut energized = HashSet::new();
        let mut paths = self.paths.clone();
        let mut dir_vec = vec![self.start_dir];
        while let Some(mut cur) = dir_vec.pop() {
            while let Some(next) = cur.next(
                &mut energized,
                &mut paths,
                row_non_zeros,
                col_non_zeros,
                self.num_rows,
                self.num_cols,
                &mut dir_vec,
            ) {
                if cur == next {
                    break;
                }
                cur = next;
            }
        }
        energized.len()
    }
}

#[aoc(day16, part2, am)]
pub fn part_2_am(input: &Arc<(Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>)>) -> usize {
    let mut reqs = Vec::new();
    for col in 0..input.1.len() {
        reqs.push(WORLD.exec_am_local(Part2 {
            data: input.clone(),
            num_rows: input.0.len(),
            num_cols: input.1.len(),
            paths: HashSet::new(),
            start_dir: Dir::Down((0, col)),
        }));
        reqs.push(WORLD.exec_am_local(Part2 {
            data: input.clone(),
            num_rows: input.0.len(),
            num_cols: input.1.len(),
            paths: HashSet::new(),
            start_dir: Dir::Up((input.0.len() - 1, col)),
        }));
    }
    for row in 0..input.0.len() {
        reqs.push(WORLD.exec_am_local(Part2 {
            data: input.clone(),
            num_rows: input.0.len(),
            num_cols: input.1.len(),
            paths: HashSet::new(),
            start_dir: Dir::Right((row, 0)),
        }));
        reqs.push(WORLD.exec_am_local(Part2 {
            data: input.clone(),
            num_rows: input.0.len(),
            num_cols: input.1.len(),
            paths: HashSet::new(),
            start_dir: Dir::Left((row, input.1.len() - 1)),
        }));
    }
    *WORLD
        .block_on(futures::future::join_all(reqs))
        .iter()
        .max()
        .unwrap()
}
