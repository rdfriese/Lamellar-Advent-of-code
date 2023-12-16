use std::collections::{HashMap, HashSet};
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
fn parse(input: &str) -> (Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>) {
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
    (row_non_zeros, col_non_zeros)
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
pub fn part_1_serial(data: &(Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>)) -> usize {
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
pub fn part_2_serial(data: &(Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>)) -> usize {
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

// #[aoc(day16, part2, serial)]
// pub fn part_2_serial(data: &str) -> usize {
//     let mut boxes = vec![HashMap::new(); 256];
//     for line in data.lines() {
//         for (*i, step) in line.split(',').enumerate() {
//             let mut global_h = 0;
//             let mut label_h: u64 = 0;
//             let bytes = step.as_bytes();
//             for j in 0..bytes.len() {
//                 match bytes[*j] {
//                     b'-' => {
//                         boxes[global_h].remove(&label_h);
//                         break;
//                     }
//                     b'=' => {
//                         boxes[global_h]
//                             .entry(label_h)
//                             .and_modify(|(_, lens)| *lens = bytes[j + 1] - 48)
//                             .or_insert((*i, bytes[j + 1] - 48));
//                         break;
//                     }
//                     _ => {
//                         global_h = ((global_h + bytes[*j] as usize) * 17) % 256;
//                         label_h = (label_h << 8) | bytes[*j] as u64;
//                     }
//                 }
//             }
//         }
//     }

//     boxes
//         .drain(..)
//         .enumerate()
//         .map(|(*i, b)| {
//             let mut vec = b.into_values().collect::<Vec<_>>();
//             vec.sort_unstable();
//             vec.iter()
//                 .enumerate()
//                 .map(|(j, (_, f))| (*i + 1) * (j + 1) * *f as usize)
//                 .sum::<usize>()
//         })
//         .sum::<usize>()
// }

// #[AmLocalData]
// struct Part1 {
//     data: Arc<Vec<Vec<u8>>>,
//     start: usize,
//     n: usize,
//     sum: Arc<AtomicUsize>,
// }

// #[local_am]
// impl LamellarAm for Part1 {
//     async fn exec() {
//         self.sum.fetch_add(
//             self.data[self.start..]
//                 .iter()
//                 .step_by(self.n)
//                 .map(|s| s.iter().fold(0, |acc, &b| ((acc + b as usize) * 17) % 256))
//                 .sum::<usize>(),
//             Ordering::SeqCst,
//         );
//     }
// }

// #[aoc_generator(day16, part1, am)]
// fn parse_input_1_am(input: &str) -> std::sync::Arc<Vec<Vec<u8>>> {
//     let mut steps = vec![];
//     input.lines().for_each(|l| {
//         l.split(',').for_each(|s| steps.push(s.as_bytes().to_vec()));
//     });
//     std::sync::Arc::new(steps)
// }

// #[aoc(day16, part1, am)]
// pub fn part_1_am(input: &std::sync::Arc<Vec<Vec<u8>>>) -> usize {
//     let num_threads = WORLD.num_threads_per_pe();
//     let sum = Arc::new(AtomicUsize::new(0));

//     (0..num_threads).for_each(|t| {
//         WORLD.exec_am_local(Part1 {
//             data: input.clone(),
//             start: t,
//             n: num_threads,
//             sum: sum.clone(),
//         });
//     });
//     WORLD.wait_all();
//     sum.load(Ordering::SeqCst)
// }

// // because the order in which the same labels are processed need to be quaranteed
// // we can't simply parallize over the steps, instead we will group the steps by label
// // and then parallelize over the label groups, unforunately this means most the processing
// // will be done serially, limiting any benefits of parallelism
// #[AmLocalData]
// struct Part2 {
//     labels: Arc<HashMap<u64, Vec<StepOp>>>,
//     start: usize,
//     n: usize,
//     boxes: Arc<Vec<Mutex<HashMap<u64, (usize, u8)>>>>,
// }

// #[local_am]
// impl LamellarAm for Part2 {
//     async fn exec() {
//         for label in self.labels.keys().skip(self.start).step_by(self.n) {
//             self.labels
//                 .get(label)
//                 .unwrap()
//                 .iter()
//                 .for_each(|op| match op {
//                     StepOp::Equal(order, global_h, fl) => {
//                         self.boxes[*global_h]
//                             .lock()
//                             .unwrap()
//                             .entry(*label)
//                             .and_modify(|(_, lens)| *lens = *fl)
//                             .or_insert((*order, *fl));
//                     }
//                     StepOp::Minus(global_h) => {
//                         self.boxes[*global_h].lock().unwrap().remove(label);
//                     }
//                 })
//         }
//     }
// }

// pub enum StepOp {
//     Equal(usize, usize, u8),
//     Minus(usize),
// }
// #[aoc_generator(day16, part2, am)]
// fn parse_input_2_am(input: &str) -> std::sync::Arc<HashMap<u64, Vec<StepOp>>> {
//     let mut steps_map = HashMap::new();
//     input.lines().for_each(|l| {
//         l.split(',').enumerate().for_each(|(*i, s)| {
//             let mut label_h: u64 = 0;
//             let mut global_h = 0;
//             let bytes = s.as_bytes();
//             for j in 0..bytes.len() {
//                 match bytes[*j] {
//                     b'-' => {
//                         steps_map
//                             .entry(label_h)
//                             .or_insert(vec![])
//                             .push(StepOp::Minus(global_h));
//                         break;
//                     }
//                     b'=' => {
//                         steps_map
//                             .entry(label_h)
//                             .or_insert(vec![])
//                             .push(StepOp::Equal(*i, global_h, bytes[j + 1] - 48));
//                         break;
//                     }
//                     _ => {
//                         global_h = ((global_h + bytes[*j] as usize) * 17) % 256;
//                         label_h = (label_h << 8) | bytes[*j] as u64;
//                     }
//                 }
//             }
//         })
//     });
//     std::sync::Arc::new(steps_map)
// }

// #[aoc(day16, part2, am)]
// pub fn part_2_am(input: &std::sync::Arc<HashMap<u64, Vec<StepOp>>>) -> usize {
//     let num_threads = WORLD.num_threads_per_pe();
//     let mut boxes = vec![];
//     for _ in 0..256 {
//         boxes.push(Mutex::new(HashMap::new()));
//     }
//     let boxes = std::sync::Arc::new(boxes);

//     (0..num_threads).for_each(|t| {
//         WORLD.exec_am_local(Part2 {
//             labels: input.clone(),
//             start: t,
//             n: num_threads,
//             boxes: boxes.clone(),
//         });
//     });
//     WORLD.wait_all();
//     boxes
//         .iter()
//         .enumerate()
//         .map(|(*i, b)| {
//             let mut vec = b.lock().unwrap().values().map(|&v| v).collect::<Vec<_>>();
//             vec.sort_unstable();
//             vec.iter()
//                 .enumerate()
//                 .map(|(j, (_, f))| (*i + 1) * (j + 1) * *f as usize)
//                 .sum::<usize>()
//         })
//         .sum::<usize>()
// }
