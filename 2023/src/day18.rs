use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};

use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
//maybe we need to return a vec of valid splits

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day18, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}
#[aoc_generator(day18)]
fn parse(input: &str) -> (Arc<Vec<(Dir, usize, Trench)>>, usize, usize) {
    let mut upper_h = 0;
    let mut upper_w = 0;
    let data = input
        .lines()
        .map(|line| {
            let mut vals = line.split_whitespace();
            let dir = vals.next().unwrap();
            let num = vals.next().unwrap().parse::<usize>().unwrap();
            let dir = match dir {
                "U" => Dir::Up,
                "D" => {
                    upper_h += num as usize;
                    Dir::Down
                }
                "L" => Dir::Left,
                "R" => {
                    upper_w += num as usize;
                    Dir::Right
                }
                _ => unreachable!(),
            };

            // let color = u32::from_str_radix(vals.next().unwrap(), 16).unwrap();
            (dir, num, Trench::Edge(0))
        })
        .collect::<Vec<_>>();
    (Arc::new(data), upper_h, upper_w)
}

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub enum Trench {
    Edge(u32),
    Interior,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Entry {
    Edge(isize),
    Line(isize, isize),
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // println!("{:?} {:?}", self, other);
        match (self, other) {
            (Entry::Edge(a), Entry::Edge(b)) => a.cmp(b),
            (Entry::Edge(a), Entry::Line(b, c)) => a.cmp(b).then_with(|| a.cmp(c)),
            (Entry::Line(a, b), Entry::Edge(c)) => a.cmp(c).then_with(|| b.cmp(c)),
            (Entry::Line(a, b), Entry::Line(c, d)) => a.cmp(c).then_with(|| b.cmp(d)),
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn add_entry(lines: &mut Vec<Entry>, new: Entry) {
    // println!("{:?}", new);
    let valid = true;
    for i in 0..lines.len() {
        match (new, lines[i]) {
            (Entry::Edge(a), Entry::Edge(b)) => {
                if a == b {
                    return; //already exists
                }
            }
            (Entry::Edge(a), Entry::Line(b, c)) => {
                if a == b || a == c {
                    return;
                }
            }
            (Entry::Line(a, b), Entry::Edge(c)) => {
                if a == c || b == c {
                    // edge exists on line
                    lines[i] = Entry::Line(a, b);
                    return;
                }
            }
            (Entry::Line(a, b), Entry::Line(c, d)) => {
                if a <= c && b >= d {
                    //new line overlaps old line
                    lines[i] = Entry::Line(a, b);
                    return;
                } else if a > c && b < d {
                    //old line overlaps new line
                    return;
                } else if a <= c && b >= c && b <= d {
                    //partial overlap
                    lines[i] = Entry::Line(a, d);
                    return;
                } else if a >= c && a <= d && b >= d {
                    lines[i] = Entry::Line(c, b);
                    return;
                }
            }
        }
    }
    lines.push(new);
}

#[aoc(day18, part1, serial)]
pub fn part_1_serial(
    (data, upper_h, upper_w): &(Arc<Vec<(Dir, usize, Trench)>>, usize, usize),
) -> isize {
    let mut sum = 0;
    let mut lines = BTreeMap::new();
    let mut lines2 = BTreeMap::new();
    let mut cur_i = 0isize;
    let mut cur_j = 0;
    // let mut min_i = 0;
    // let mut min_j = 0;
    for (dir, num, __) in data.iter() {
        // println!("{lines:?}");
        // println!("{dir:?}");
        let num = *num as isize;
        match dir {
            Dir::Up => {
                for i in cur_i as isize - num..cur_i as isize {
                    add_entry(
                        &mut lines.entry(i).or_insert(Vec::new()),
                        Entry::Edge(cur_j),
                    );
                    lines2
                        .entry(i)
                        .or_insert(Vec::new())
                        .push(Entry::Edge(cur_j));
                }
                cur_i -= num as isize;
            }
            Dir::Down => {
                for i in cur_i as isize..cur_i as isize + num {
                    add_entry(
                        &mut lines.entry(i).or_insert(Vec::new()),
                        Entry::Edge(cur_j),
                    );
                    lines2
                        .entry(i)
                        .or_insert(Vec::new())
                        .push(Entry::Edge(cur_j));
                }
                cur_i += num as isize;
            }
            Dir::Left => {
                add_entry(
                    &mut lines.entry(cur_i as isize).or_insert(Vec::new()),
                    Entry::Line(cur_j - num as isize, cur_j),
                );
                lines2
                    .entry(cur_i as isize)
                    .or_insert(Vec::new())
                    .push(Entry::Line(cur_j - num as isize, cur_j));
                cur_j -= num as isize;
            }
            Dir::Right => {
                add_entry(
                    &mut lines.entry(cur_i as isize).or_insert(Vec::new()),
                    Entry::Line(cur_j, cur_j + num as isize),
                );
                lines2
                    .entry(cur_i as isize)
                    .or_insert(Vec::new())
                    .push(Entry::Line(cur_j, cur_j + num as isize));
                cur_j += num as isize;
            }
        }
        // println!("{lines:?}");
        // println!("");
    }
    for (i, line) in lines.iter_mut() {
        line.sort();
        println!("{i} {:?}", line);
        let mut line2 = lines2.get(i).unwrap().clone();
        line2.sort();
        println!("{i} {:?}", line2);
        // for window in lines.windows(2) {
        let mut i = 0;
        let mut tempsum = 0;
        while i < line.len() {
            if i % 2 == 0 {
                println!("{:?}", line[i]);
                match line[i] {
                    Entry::Line(a, b) => {
                        tempsum += (b - a) + 1;
                        i += 1;
                        if i < line.len() {
                            match line[i] {
                                Entry::Line(c, d) => {
                                    tempsum += d - b; // already accounted for inclusive range
                                }
                                Entry::Edge(c) => {
                                    tempsum += c - b; //already accounted for inclusive range
                                }
                            }
                        }
                    }
                    Entry::Edge(a) => {
                        i += 1; //assume that if we are on an even index as an edge there must be at least one other edge
                        match line[i] {
                            Entry::Line(b, c) => {
                                tempsum += c - b;
                                tempsum += (b - a) + 1;
                            }
                            Entry::Edge(b) => {
                                tempsum += (b - a) + 1;
                            }
                        }
                    }
                }
                i += 1;
            } else {
                //only way we get here is if it is a line
                match line[i] {
                    Entry::Line(a, b) => {
                        tempsum += (b - a) + 1;
                    }
                    Entry::Edge(a) => {
                        println!(" can we be here? {:?}", line[i]);
                    }
                }
                i += 1;
            }
        }
        sum += tempsum;
        println!("{} {}", tempsum, sum);
    }
    sum
}
