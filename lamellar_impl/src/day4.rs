use crate::WORLD;
use aoc_runner_derive::aoc;
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    str::{self, Split},
    sync::atomic::{AtomicU32, Ordering},
};

fn get_matches<'a>(mut numbers: Split<'a, &str>) -> usize {
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
    winning.intersection(&my_numbers).count()
}

#[AmLocalData(Debug)]
struct Part1 {
    line: String,
    sum: Darc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec() {
        let mut line = self.line.split(":");
        let _game = line.next();
        let numbers = line.next().expect("properly formed line").split("|");
        let num_matches = get_matches(numbers) as u32;
        self.sum
            .fetch_add(2_u32.pow(num_matches - 1), Ordering::Relaxed);
    }
}

#[AmLocalData(Debug)]
struct Part2Slow {
    games: Darc<Vec<String>>,
    line: usize,
    sum: Darc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part2Slow {
    async fn exec() {
        let mut line = self.games[self.line].split(":");
        let _game = line.next();
        let numbers = line.next().expect("properly formed line").split("|");
        let num_matches = get_matches(numbers);
        for new_line in self.line + 1..self.line + 1 + num_matches {
            lamellar::world.exec_am_local(Part2Slow {
                games: self.games.clone(),
                line: new_line,
                sum: self.sum.clone(),
            });
        }
        self.sum.fetch_add(1, Ordering::Relaxed);
    }
}
#[AmLocalData(Debug)]
struct Part2Fast {
    games: Darc<Vec<String>>,
    line: usize,
    cards: LocalRwDarc<HashMap<usize, usize>>,
}

#[local_am]
impl LamellarAm for Part2Fast {
    async fn exec() {
        let mut my_cards = HashMap::new();
        let mut max_line = self.line + 1;
        let mut cur_line = self.line;

        while cur_line < max_line {
            let copies = *my_cards.entry(cur_line).or_insert(1);
            let mut line = self.games[cur_line].split(":");
            let _game = line.next();
            let numbers = line.next().expect("properly formed line").split("|");
            let num_matches = get_matches(numbers);
            let end_line = cur_line + 1 + num_matches;
            for j in cur_line + 1..end_line {
                *my_cards.entry(j).or_insert(0) += copies; //the one is the original card and then  add how many copies of the card there are
            }
            if max_line < end_line {
                max_line = end_line
            }
            cur_line += 1;
        }
        let mut cards = self.cards.write().await;
        for (k, v) in my_cards {
            *cards.entry(k).or_insert(0) += v;
        }
    }
}

#[aoc(day4, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

#[aoc(day4, part1, am)]
pub fn part_1_am(input: &str) -> u32 {
    let sum = Darc::new(WORLD.team(), AtomicU32::new(0)).unwrap();
    for line in input.lines() {
        WORLD.exec_am_local(Part1 {
            line: line.to_string(),
            sum: sum.clone(),
        });
    }
    WORLD.wait_all();
    sum.load(Ordering::SeqCst)
}

// #[aoc(day4, part2, slow)]
// pub fn part_2_slow(input: &str) -> u32 {
//     let games = Darc::new(
//         WORLD.team(),
//         input
//             .lines()
//             .map(|line| line.to_string())
//             .collect::<Vec<_>>(),
//     )
//     .unwrap();
//     let sum = Darc::new(WORLD.team(), AtomicU32::new(0)).unwrap();
//     for i in 0..games.len() {
//         WORLD.exec_am_local(Part2Slow {
//             games: games.clone(),
//             line: i,
//             sum: sum.clone(),
//         });
//     }
//     WORLD.wait_all();
//     sum.load(Ordering::SeqCst)
// }

#[aoc(day4, part2, fast)]
pub fn part_2_fast(input: &str) -> u32 {
    let games = Darc::new(
        WORLD.team(),
        input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let cards: LocalRwDarc<HashMap<_, usize>> =
        LocalRwDarc::new(WORLD.team(), HashMap::new()).unwrap();

    for i in 0..games.len() {
        WORLD.exec_am_local(Part2Fast {
            games: games.clone(),
            line: i,
            cards: cards.clone(),
        });
    }
    WORLD.wait_all();
    WORLD
        .block_on(cards.read())
        .iter()
        .map(|(_, v)| v)
        .sum::<usize>() as u32
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
        for new_line in i + 1..i + 1 + num_matches {
            *cards.entry(new_line).or_insert(1) += copies; //the one is the original card and then  add how many copies of the card there are
        }
    }
    cards.iter().map(|(_, v)| v).sum::<usize>() as u32
}
