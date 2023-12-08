use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
use lamellar::array::prelude::*;
use lamellar::darc::prelude::*;
use num::integer::Integer;

const LAST_DIGIT: u16 = 676; // 26.pow(2)
const MID_DIGIT: u16 = 26; // 26.pow(1)
const FIRST_DIGIT: u16 = 1; // 26.pow(0)
const MAX_NUM: usize = 17575; // 26.pow(3)

#[aoc_generator(day8, part1)]
fn parse_part1(input: &str) -> (Vec<usize>, Vec<[u16; 2]>) {
    let mut data = vec![[0_u16, 0_u16]; MAX_NUM + 1];
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 }) //we just assume the input is only L and R
        .collect::<Vec<usize>>();
    lines.next();
    for line in lines {
        let u8_line = line.as_bytes();
        let index = (u8_line[0] - 65) as u16 * LAST_DIGIT
            + (u8_line[1] - 65) as u16 * MID_DIGIT
            + (u8_line[2] - 65) as u16 * FIRST_DIGIT;
        let left = (u8_line[7] - 65) as u16 * LAST_DIGIT
            + (u8_line[8] - 65) as u16 * MID_DIGIT
            + (u8_line[9] - 65) as u16 * FIRST_DIGIT;
        let right = (u8_line[12] - 65) as u16 * LAST_DIGIT
            + (u8_line[13] - 65) as u16 * MID_DIGIT
            + (u8_line[14] - 65) as u16 * FIRST_DIGIT;
        data[index as usize] = [left, right];
    }
    (directions, data)
}

#[aoc_generator(day8, part2)]
fn parse_part2(input: &str) -> (Vec<usize>, Vec<usize>, Vec<[u16; 3]>) {
    let mut data = vec![[0_u16, 0_u16, 0_u16]; MAX_NUM + 1];
    let mut starts = vec![];
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 }) //we just assume the input is only L and R
        .collect::<Vec<usize>>();
    lines.next();
    for line in lines {
        let u8_line = line.as_bytes();
        let first = (u8_line[2] - 65) as u16 * FIRST_DIGIT;
        let index =
            (u8_line[0] - 65) as u16 * LAST_DIGIT + (u8_line[1] - 65) as u16 * MID_DIGIT + first;
        if first == 0 {
            starts.push(index as usize);
        }
        let z = if first == 25 { 1 } else { 0 };
        let left = (u8_line[7] - 65) as u16 * LAST_DIGIT
            + (u8_line[8] - 65) as u16 * MID_DIGIT
            + (u8_line[9] - 65) as u16 * FIRST_DIGIT;
        let right = (u8_line[12] - 65) as u16 * LAST_DIGIT
            + (u8_line[13] - 65) as u16 * MID_DIGIT
            + (u8_line[14] - 65) as u16 * FIRST_DIGIT;
        data[index as usize] = [left, right, z as u16];
    }
    (directions, starts, data)
}

#[aoc(day8, part1, A_INIT_WORLD)]
pub fn part_1(_: &(Vec<usize>, Vec<[u16; 2]>)) -> u32 {
    WORLD.num_pes() as u32
}

#[aoc(day8, part1, serial)]
pub fn part_1_serial((directions, data): &(Vec<usize>, Vec<[u16; 2]>)) -> u32 {
    let mut cur_index = 0;
    directions
        .iter()
        .cycle()
        .map_while(|&dir| {
            cur_index = data[cur_index][dir as usize] as usize;
            (cur_index != MAX_NUM).then_some(())
        })
        .count() as u32
        + 1
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

#[aoc(day8, part2, serial)]
pub fn part_2_serial(
    (directions, starts, data): &(Vec<usize>, Vec<usize>, Vec<[u16; 3]>),
) -> usize {
    starts
        .iter()
        .map(|i| {
            let mut i = *i;
            directions
                .iter()
                .cycle()
                .map_while(|&dir| {
                    i = data[i][dir as usize] as usize;
                    (data[i][2] as usize != 1).then_some(())
                })
                .count() as usize
                + 1
        })
        .fold(1, |acc, x| lcm(acc, x))
}

//we need to tell lamellar that we want this type to be used in a lamellar array
#[AmData(Default, Debug, ArrayOps)]
pub struct Coordinates {
    l: u16,
    r: u16,
}

impl std::ops::Index<usize> for Coordinates {
    type Output = u16;
    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.l
        } else {
            &self.r
        }
    }
}

#[aoc_generator(day8, part1, lamellar)]
fn parse_part1_lamellar(input: &str) -> (Vec<usize>, ReadOnlyArray<Coordinates>) {
    // let mut data = vec![[0_u16, 0_u16]; MAX_NUM + 1];
    let data = LocalLockArray::new(&*WORLD, MAX_NUM + 1, Distribution::Block);
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 }) //we just assume the input is only L and R
        .collect::<Vec<usize>>();
    lines.next();
    let mut indices = vec![];
    let mut coordinates = vec![];
    for line in lines {
        let u8_line = line.as_bytes();
        let index = (u8_line[0] - 65) as u16 * LAST_DIGIT
            + (u8_line[1] - 65) as u16 * MID_DIGIT
            + (u8_line[2] - 65) as u16 * FIRST_DIGIT;
        let left = (u8_line[7] - 65) as u16 * LAST_DIGIT
            + (u8_line[8] - 65) as u16 * MID_DIGIT
            + (u8_line[9] - 65) as u16 * FIRST_DIGIT;
        let right = (u8_line[12] - 65) as u16 * LAST_DIGIT
            + (u8_line[13] - 65) as u16 * MID_DIGIT
            + (u8_line[14] - 65) as u16 * FIRST_DIGIT;
        // we could very easily just call the following
        // data.store(index as usize, Coordinates { l: left, r: right });
        // but... currently single array ops in Lamellar are not terribly optimized so we prefer
        // to batch operations
        indices.push(index as usize);
        coordinates.push(Coordinates { l: left, r: right });
    }
    println!("{:?} {:?}", indices, coordinates);
    WORLD.block_on(data.batch_store(indices, coordinates)); //Lamellar will effeciently aggregate and apply the operation

    (directions, data.into_read_only())
}

#[aoc(day8, part1, lamellar)]
pub fn part_1_lamellar_array(
    (directions, data): &(Vec<usize>, ReadOnlyArray<Coordinates>),
) -> usize {
    WORLD.block_on(async move {
        let mut cnt = 0;
        let mut cur_index = 0;
        for dir in directions.iter().cycle() {
            cnt += 1;
            cur_index = data.load(cur_index).await[*dir as usize] as usize;
            if cur_index == MAX_NUM {
                break;
            }
        }
        cnt
    })
}
