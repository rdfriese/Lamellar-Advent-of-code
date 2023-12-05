use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

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
struct Part2 {
    games: Darc<Vec<String>>,
    line: usize,
    sum: Darc<AtomicU32>,
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
    games: Darc<Vec<String>>,
    line: usize,
    sum: Darc<AtomicU32>,
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

#[AmLocalData(Debug)]
struct Part2V3 {
    games: Darc<Vec<String>>,
    line: usize,
    // cards: LocalRwDarc<HashMap<usize, usize>>,
    cards: Darc<Mutex<HashMap<usize, usize>>>,
}

#[local_am]
impl LamellarAm for Part2V3 {
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
            println!(
                "[{}] cur_line: {}, max_line: {}",
                self.line, cur_line, max_line
            );
        }
        // let mut cards = self.cards.write().await;
        let mut cards = self.cards.lock().unwrap();
        println!("[{}] my_cards: {:?}", self.line, my_cards);
        for (k, v) in my_cards {
            *cards.entry(k).or_insert(0) += v;
        }
        println!("[{}] cards: {:?}", self.line, cards);
    }
}

pub fn part_1(world: &LamellarWorld) {
    let f = File::open("inputs/day4_test.txt").unwrap();
    let sum = Darc::new(world, AtomicU32::new(0)).unwrap();
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
    let f = File::open("inputs/day4_test.txt").unwrap();
    let games = Darc::new(
        world,
        BufReader::new(&f)
            .lines()
            .into_iter()
            .map(|line| line.expect("line exists"))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let sum = Darc::new(world, AtomicU32::new(0)).unwrap();
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
    let f = File::open("inputs/day4_test.txt").unwrap();
    let games = Darc::new(
        world,
        BufReader::new(&f)
            .lines()
            .into_iter()
            .map(|line| line.expect("line exists"))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let sum = Darc::new(world, AtomicU32::new(0)).unwrap();
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

pub fn part_2_v3(world: &LamellarWorld) {
    let f = File::open("inputs/day4_test.txt").unwrap();
    let games = Darc::new(
        world,
        BufReader::new(&f)
            .lines()
            .into_iter()
            .map(|line| line.expect("line exists"))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    // let cards: LocalRwDarc<HashMap<_, usize>> = LocalRwDarc::new(world, HashMap::new()).unwrap();
    let cards = Darc::new(world, Mutex::new(HashMap::new())).unwrap();

    for i in 0..games.len() {
        world.exec_am_local(Part2V3 {
            games: games.clone(),
            line: i,
            cards: cards.clone(),
        });
    }
    world.wait_all();
    println!("here");
    let sum: usize = cards.lock().unwrap().iter().map(|(_, v)| v).sum();
    println!("Sum: {sum}");
}

// the serial version below is significantly faster,
// but the dependency of a card on the cards occuring before
// it limits parallelism that an lamellar approach can represent
pub fn part_2_serial(_world: &LamellarWorld) {
    let f = File::open("inputs/day4_test.txt").unwrap();
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
