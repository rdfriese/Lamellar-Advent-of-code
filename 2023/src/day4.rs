use crate::WORLD;
use aoc_runner_derive::{aoc, aoc_generator};
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    str::{self, Split},
    sync::atomic::{AtomicU32, Ordering},
};

// need to construct a LamellarWorld
// but only one can be constructed per execution
// so simply use this to initialize to once_cell containing
// the world so as not to affect the timings of the actual solutions
#[aoc(day4, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

fn get_matches<'a>(mut numbers: Split<'a, &str>) -> u32 {
    let winning = numbers
        .next()
        .expect("properly formed line")
        .trim()
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<HashSet<_>>();
    let my_numbers = numbers
        .next()
        .expect("properly formed line")
        .trim()
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<HashSet<_>>();
    winning.intersection(&my_numbers).count() as u32
}

#[aoc(day4, part1, serial)]
pub fn part_1_serial(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(":");
            let _game = line.next();
            let numbers = line.next().expect("properly formed line").split("|");
            let num_matches = get_matches(numbers) as u32;
            2_u32.pow(num_matches - 1) as u32
        })
        .sum::<u32>()
}

#[aoc(day4, part2, serial)]
pub fn part_2_serial(input: &str) -> u32 {
    let mut cards = HashMap::<usize, usize>::new();
    for (i, line) in input.lines().enumerate() {
        let copies = *cards.entry(i).or_insert(1); //one for original card
        let mut line = line.split(":");
        let _game = line.next();
        let numbers = line.next().expect("properly formed line").split("|");
        let num_matches = get_matches(numbers);
        for new_line in i + 1..i + 1 + num_matches as usize {
            *cards.entry(new_line).or_insert(1) += copies; //the one is the original card and then  add how many copies of the card there are
        }
    }
    cards.iter().map(|(_, v)| v).sum::<usize>() as u32
}

#[AmLocalData(Debug)]
struct Part1 {
    lines: Darc<Vec<String>>,
    start: usize,
    n: usize,
    sum: Darc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() {
        self.sum.fetch_add(
            self.lines[self.start..]
                .iter()
                .step_by(self.n)
                .map(|line| {
                    let mut line = line.split(":");
                    let _game = line.next();
                    let numbers = line.next().expect("properly formed line").split("|");
                    2_u32.pow(get_matches(numbers) as u32 - 1)
                })
                .sum(),
            Ordering::Relaxed,
        );
    }
}

#[aoc_generator(day4, part1, am)]
fn parse_am(input: &str) -> Darc<Vec<String>> {
    Darc::new(
        WORLD.team(),
        input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[aoc(day4, part1, am)]
pub fn part_1_am(input: &Darc<Vec<String>>) -> u32 {
    let sum = Darc::new(WORLD.team(), AtomicU32::new(0)).unwrap();
    let num_threads = WORLD.num_threads_per_pe();
    for t in 0..num_threads {
        WORLD.exec_am_local(Part1 {
            lines: input.clone(),
            start: t,
            n: num_threads,
            sum: sum.clone(),
        });
    }
    WORLD.wait_all();
    sum.load(Ordering::SeqCst)
}

#[AmLocalData(Debug)]
struct Part2 {
    lines: Darc<Vec<String>>,
    start: usize,
    n: usize,
    cards: LocalRwDarc<HashMap<usize, usize>>,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec() {
        let mut my_cards = HashMap::new();
        let mut max_line = 0;
        for i in self.start..self.start + self.n {
            let copies = *my_cards.entry(i).or_insert(1); //one for original card
            let mut line = self.lines[i].split(":");
            let _game = line.next();
            let numbers = line.next().expect("properly formed line").split("|");
            let num_matches = get_matches(numbers);
            let end_line = i + 1 + num_matches as usize;
            for new_line in i + 1..std::cmp::min(end_line, self.start + self.n) {
                *my_cards.entry(new_line).or_insert(1) += copies; //the one is the original card and then  add how many copies of the card there are
            }
            for new_line in std::cmp::min(end_line, self.start + self.n)..end_line {
                *my_cards.entry(new_line).or_insert(0) += copies;
            }
            if max_line < end_line {
                max_line = end_line;
            }
        }
        let mut cur_line = self.start + self.n;
        while cur_line < max_line {
            let copies = *my_cards.entry(cur_line).or_insert(1);
            let mut line = self.lines[cur_line].split(":");
            let _game = line.next();
            let numbers = line.next().expect("properly formed line").split("|");
            let num_matches = get_matches(numbers);
            let end_line = cur_line + 1 + num_matches as usize;
            for j in cur_line + 1..end_line {
                *my_cards.entry(j).or_insert(0) += copies; //the one is the original card and then  add how many copies of the card there are
            }
            if max_line < end_line {
                max_line = end_line;
            }
            cur_line += 1;
        }
        let mut cards = self.cards.write().await;
        for (k, v) in my_cards {
            *cards.entry(k).or_insert(0) += v;
        }
    }
}

#[aoc_generator(day4, part2, am)]
fn parse_am_2(input: &str) -> Darc<Vec<String>> {
    Darc::new(
        WORLD.team(),
        input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[aoc(day4, part2, am)]
pub fn part_2(input: &Darc<Vec<String>>) -> u32 {
    let cards: LocalRwDarc<HashMap<_, usize>> =
        LocalRwDarc::new(WORLD.team(), HashMap::new()).unwrap();

    let num_threads = WORLD.num_threads_per_pe();
    let num_lines_per_thread = std::cmp::max(1, input.len() / num_threads); //for the test inputs
    input
        .chunks(num_lines_per_thread)
        .enumerate()
        .for_each(|(i, d)| {
            WORLD.exec_am_local(Part2 {
                lines: input.clone(),
                start: i * num_lines_per_thread,
                n: d.len(),
                cards: cards.clone(),
            });
        });
    WORLD.wait_all();
    WORLD
        .block_on(cards.read())
        .iter()
        .map(|(_, v)| v)
        .sum::<usize>() as u32
}
