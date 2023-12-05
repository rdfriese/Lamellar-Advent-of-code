use lamellar::active_messaging::prelude::*;
// use lamellar::darc::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    str::{self, Split},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
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
    sum: Arc<AtomicU32>,
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
struct Part2 {
    games: Arc<Vec<String>>,
    line: usize,
    sum: Arc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec() {
        let mut line = self.games[self.line].split(":");
        let _game = line.next();
        let numbers = line.next().expect("properly formed line").split("|");
        let num_matches = get_matches(numbers);
        for new_line in self.line + 1..self.line + 1 + num_matches {
            lamellar::world.exec_am_local(Part2 {
                games: self.games.clone(),
                line: new_line,
                sum: self.sum.clone(),
            });
        }
        self.sum.fetch_add(1, Ordering::Relaxed);
    }
}

#[AmLocalData(Debug)]
struct Part2V2 {
    games: Arc<Vec<String>>,
    line: usize,
    sum: Arc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part2V2 {
    async fn exec() {
        let mut my_cnt = 0;
        let mut lines = vec![self.line]; // change to queue

        while let Some(new_line) = lines.pop() {
            let mut line = self.games[new_line].split(":");
            let _game = line.next();
            let numbers = line.next().expect("properly formed line").split("|");
            let num_matches = get_matches(numbers);
            lines.extend(new_line + 1..new_line + 1 + num_matches);
            my_cnt += 1;
        }
        self.sum.fetch_add(my_cnt, Ordering::Relaxed);
    }
}

pub fn part_1(world: &LamellarWorld) {
    let f = File::open("inputs/day4.txt").unwrap();
    let sum = Arc::new(AtomicU32::new(0));
    for line in BufReader::new(&f).lines().into_iter() {
        world.exec_am_local(Part1 {
            line: line.expect("line exists"),
            sum: sum.clone(),
        });
    }
    world.wait_all();
    println!("Sum: {:?}", sum.load(Ordering::SeqCst));
}

pub fn part_2(world: &LamellarWorld) {
    let f = File::open("inputs/day4.txt").unwrap();
    let games = Arc::new(
        BufReader::new(&f)
            .lines()
            .into_iter()
            .map(|line| line.expect("line exists"))
            .collect::<Vec<_>>(),
    );
    let sum = Arc::new(AtomicU32::new(0));
    for i in 0..games.len() {
        world.exec_am_local(Part2 {
            games: games.clone(),
            line: i,
            sum: sum.clone(),
        });
    }
    world.wait_all();
    println!("Sum: {:?}", sum.load(Ordering::SeqCst));
}

pub fn part_2_v2(world: &LamellarWorld) {
    let f = File::open("inputs/day4.txt").unwrap();
    let games = Arc::new(
        BufReader::new(&f)
            .lines()
            .into_iter()
            .map(|line| line.expect("line exists"))
            .collect::<Vec<_>>(),
    );
    let sum = Arc::new(AtomicU32::new(0));
    for i in 0..games.len() {
        world.exec_am_local(Part2V2 {
            games: games.clone(),
            line: i,
            sum: sum.clone(),
        });
    }
    world.wait_all();
    println!("Sum: {:?}", sum.load(Ordering::SeqCst));
}

// this is a case where the lamellar approach likely does make much sense
// and a more efficent way is provided
pub fn part_2_serial(_world: &LamellarWorld) {
    let f = File::open("inputs/day4.txt").unwrap();
    // let sum = Arc::new(AtomicU32::new(0));

    let mut cards = HashMap::<usize, usize>::new();
    for (i, line) in BufReader::new(&f)
        .lines()
        .into_iter()
        .map(|line| line.expect("line exists"))
        .enumerate()
    {
        let copies = *cards.entry(i).or_insert(1); //one for original card
        let mut line = line.split(":");
        let _game = line.next();
        let numbers = line.next().expect("properly formed line").split("|");
        let num_matches = get_matches(numbers);
        for new_line in i + 1..i + 1 + num_matches {
            *cards.entry(new_line).or_insert(1) += copies; //the one is the original card and then  add how many copies of the card there are
        }
    }
    let sum: usize = cards.iter().map(|(_, v)| v).sum();
    println!("Sum: {sum}");
}
