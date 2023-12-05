use lamellar::active_messaging::prelude::*;
// use lamellar::darc::prelude::*;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    str,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

fn check_left(line: &[u8], i: usize) -> u32 {
    // println!("\tcheck left {i}");
    let mut start_i = i;
    while start_i > 0 {
        if !(line[start_i - 1] as char).is_numeric() {
            break;
        }
        start_i -= 1;
    }
    // println!("\t{start_i}-{i} {:?}", str::from_utf8(&line[start_i..i]));
    str::from_utf8(&line[start_i..i])
        .expect("only ascii")
        .parse::<u32>()
        .unwrap_or(0)
}

fn check_right(line: &[u8], i: usize) -> u32 {
    // println!("\tcheck right {i}");
    let mut end_i = i;
    while end_i < line.len() - 1 {
        if !(line[end_i + 1] as char).is_numeric() {
            break;
        }
        end_i += 1;
    }
    // println!("\t{i}-{end_i} {:?}", str::from_utf8(&line[i + 1..=end_i]));
    str::from_utf8(&line[i + 1..=end_i])
        .expect("only ascii")
        .parse::<u32>()
        .ok()
        .unwrap_or(0)
}

// this is when we are checking above or below
fn check_center(line: &[u8], i: usize) -> (u32, u32) {
    // println!("\tcheck center {i}");
    if !(line[i] as char).is_numeric() {
        //not one long number so check left and right diagonals
        return (check_left(line, i), check_right(line, i));
    } else {
        let mut start_i: usize = i;
        let mut end_i = i;
        while start_i > 0 {
            if !(line[start_i - 1] as char).is_numeric() {
                break;
            }
            start_i -= 1;
        }
        while end_i < line.len() - 1 {
            if !(line[end_i + 1] as char).is_numeric() {
                break;
            }
            end_i += 1;
        }
        // println!(
        //     "\t{start_i}-{end_i} {:?}",
        //     str::from_utf8(&line[start_i..=end_i])
        // );
        (
            str::from_utf8(&line[start_i..=end_i])
                .expect("only ascii")
                .parse::<u32>()
                .ok()
                .unwrap_or(0),
            0,
        )
    }
}

#[AmLocalData(Debug)]
struct Part1 {
    schematic: Arc<Vec<Vec<u8>>>, //store as bytes for easy indexing, assuming input is only ascii
    line: usize,
    sum: Arc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() {
        let mut my_sum = 0;
        for (i, c) in self.schematic[self.line]
            .iter()
            .enumerate()
            .map(|(i, c)| (i, *c as char))
        {
            if !c.is_numeric() && c != '.' {
                // check line above me
                if self.line != 0 {
                    let center = check_center(&self.schematic[self.line - 1], i);
                    my_sum += center.0 + center.1;
                }
                // check my line
                my_sum += check_left(&self.schematic[self.line], i)
                    + check_right(&self.schematic[self.line], i);
                // check below me
                if self.line != self.schematic.len() - 1 {
                    let center = check_center(&self.schematic[self.line + 1], i);
                    my_sum += center.0 + center.1;
                }
            }
        }
        self.sum.fetch_add(my_sum, Ordering::Relaxed);
    }
}

#[AmLocalData(Debug)]
struct Part2 {
    schematic: Arc<Vec<Vec<u8>>>, //store as bytes for easy indexing, assuming input is only ascii
    line: usize,
    sum: Arc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec() {
        let mut my_sum = 0;
        for (i, c) in self.schematic[self.line]
            .iter()
            .enumerate()
            .map(|(i, c)| (i, *c as char))
        {
            if !c.is_numeric() && c != '.' {
                let mut nums = vec![];
                // check line above me
                if self.line != 0 {
                    let center = check_center(&self.schematic[self.line - 1], i);
                    if center.0 != 0 {
                        nums.push(center.0);
                    }
                    if center.1 != 0 {
                        nums.push(center.1);
                    }
                }
                // check my line
                let left = check_left(&self.schematic[self.line], i);
                if left != 0 {
                    nums.push(left);
                }
                let right = check_right(&self.schematic[self.line], i);
                if right != 0 {
                    nums.push(right);
                }
                // check below me
                if self.line != self.schematic.len() - 1 {
                    let center = check_center(&self.schematic[self.line + 1], i);
                    if center.0 != 0 {
                        nums.push(center.0);
                    }
                    if center.1 != 0 {
                        nums.push(center.1);
                    }
                }
                if nums.len() == 2 {
                    my_sum += nums[0] * nums[1];
                }
            }
        }
        self.sum.fetch_add(my_sum, Ordering::Relaxed);
    }
}

pub fn part_1(world: &LamellarWorld) {
    let f = File::open("inputs/day3.txt").unwrap();
    let schematic = Arc::new(
        BufReader::new(&f)
            .lines()
            .into_iter()
            .map(|line| line.expect("line exists").into_bytes())
            .collect::<Vec<_>>(),
    );
    let sum = Arc::new(AtomicU32::new(0));
    for i in 0..schematic.len() {
        world.exec_am_local(Part1 {
            schematic: schematic.clone(),
            line: i,
            sum: sum.clone(),
        });
    }
    world.wait_all();
    println!("Sum: {:?}", sum.load(Ordering::SeqCst));
}

pub fn part_2(world: &LamellarWorld) {
    let f = File::open("inputs/day3.txt").unwrap();
    let schematic = Arc::new(
        BufReader::new(&f)
            .lines()
            .into_iter()
            .map(|line| line.expect("line exists").into_bytes())
            .collect::<Vec<_>>(),
    );
    let sum = Arc::new(AtomicU32::new(0));
    for i in 0..schematic.len() {
        world.exec_am_local(Part2 {
            schematic: schematic.clone(),
            line: i,
            sum: sum.clone(),
        });
    }
    world.wait_all();
    println!("Sum: {:?}", sum.load(Ordering::SeqCst));
}

// pub fn part_2_task_group(world: &LamellarWorld) {
//     let f = File::open("inputs/day2.txt").unwrap();
//     let my_pe = world.my_pe();
//     let mut tg = typed_am_group! {Part2, world};
//     for line in BufReader::new(&f).lines() {
//         tg.add_am_pe(
//             my_pe,
//             Part2 {
//                 line: line.unwrap(),
//             },
//         )
//     }
//     let sum: u32 = world
//         .block_on(tg.exec())
//         .iter()
//         .map(|x| {
//             if let AmGroupResult::Pe(pe, val) = x {
//                 *val
//             } else {
//                 0
//             }
//         })
//         .sum();
//     println!("Sum: {sum}");
// }
