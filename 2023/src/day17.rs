use std::collections::{BinaryHeap, HashMap, HashSet};
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
#[aoc(day17, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}
#[aoc_generator(day17)]
fn parse(input: &str) -> Arc<Vec<Vec<Block>>> {
    let data = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, e)| Block {
                    pos: (i, j),
                    prev_pos: (i, j),
                    weight: *e - 48 as u8,
                    cost: u32::MAX,
                    dir: DirCnt::None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Arc::new(data)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum DirCnt {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
    None,
}

impl DirCnt {
    fn is_up(&self) -> bool {
        match self {
            DirCnt::Up(_) => true,
            _ => false,
        }
    }
    fn is_down(&self) -> bool {
        match self {
            DirCnt::Down(_) => true,
            _ => false,
        }
    }
    fn is_left(&self) -> bool {
        match self {
            DirCnt::Left(_) => true,
            _ => false,
        }
    }
    fn is_right(&self) -> bool {
        match self {
            DirCnt::Right(_) => true,

            _ => false,
        }
    }
    fn get_dir((i1, j1): (usize, usize), (i2, j2): (usize, usize)) -> Self {
        if i1 == i2 {
            if j1 < j2 {
                DirCnt::Right(0)
            } else {
                DirCnt::Left(0)
            }
        } else {
            if i1 < i2 {
                DirCnt::Down(0)
            } else {
                DirCnt::Up(0)
            }
        }
    }
    fn next_dir(&self) -> Self {
        match self {
            DirCnt::Up(x) => DirCnt::Up(*x + 1),
            DirCnt::Down(x) => DirCnt::Down(*x + 1),
            DirCnt::Left(x) => DirCnt::Left(*x + 1),
            DirCnt::Right(x) => DirCnt::Right(*x + 1),
            DirCnt::None => DirCnt::None,
        }
    }
    fn cnt(&self) -> usize {
        match self {
            DirCnt::Up(x) => *x,
            DirCnt::Down(x) => *x,
            DirCnt::Left(x) => *x,
            DirCnt::Right(x) => *x,
            DirCnt::None => 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    pos: (usize, usize),
    prev_pos: (usize, usize),
    weight: u8,
    cost: u32,
    dir: DirCnt,
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.pos == other.pos
    }
}
impl Eq for Block {}

fn get_possible_neighbors(cur: &Block, num_rows: usize, num_cols: usize) -> Vec<(usize, usize)> {
    let (i, j) = cur.pos;
    let (p_i, p_j) = cur.prev_pos;
    let mut res = Vec::new();
    if i > 0 && p_i != i - 1 {
        res.push((i - 1, j));
    }
    if j > 0 && p_j != j - 1 {
        res.push((i, j - 1));
    }
    if i < num_rows - 1 && p_i != i + 1 {
        res.push((i + 1, j));
    }
    if j < num_cols - 1 && p_j != j + 1 {
        res.push((i, j + 1));
    }
    res
}

fn same_direction(
    cur: Block,
    new_pos: (usize, usize),
    data: &Vec<Vec<Block>>,
    g_data: &mut GraphData,
) {
    let next_dir = cur.dir.next_dir();
    let mut next = if let Some(next) = g_data.min_cost_cache.get(&(new_pos, next_dir)) {
        next.clone()
    } else {
        data[new_pos.0 as usize][new_pos.1 as usize]
    };
    let temp_cost = cur.cost + next.weight as u32;
    if temp_cost < next.cost {
        next.prev_pos = cur.pos;
        next.cost = temp_cost;
        next.dir = next_dir;
        g_data
            .min_cost_cache
            .insert((next.pos, next.dir), next.clone());
        if !g_data.heap_contents.contains(&(next.pos, next.dir)) {
            g_data.heap_contents.insert((next.pos, next.dir));
            g_data.heap.push(next.clone());
        }
    }
}

fn new_direction(
    cur: Block,
    new_pos: (usize, usize),
    data: &Vec<Vec<Block>>,
    g_data: &mut GraphData,
) {
    let next_dir = DirCnt::get_dir(cur.pos, new_pos);
    let mut next = if let Some(next) = g_data.min_cost_cache.get(&(new_pos, next_dir)) {
        next.clone()
    } else {
        data[new_pos.0 as usize][new_pos.1 as usize]
    };
    let temp_cost = cur.cost + next.weight as u32;
    if temp_cost < next.cost {
        next.prev_pos = cur.pos;
        next.cost = temp_cost;
        next.dir = next_dir;
        g_data
            .min_cost_cache
            .insert((next.pos, next.dir), next.clone());
        if !g_data.heap_contents.contains(&(next.pos, next.dir)) {
            g_data.heap_contents.insert((next.pos, next.dir));
            g_data.heap.push(next.clone());
        }
    }
}

#[derive(Clone)]
struct GraphData {
    heap: BinaryHeap<Block>,
    heap_contents: HashSet<((usize, usize), DirCnt)>,
    min_cost_cache: HashMap<((usize, usize), DirCnt), Block>,
}

#[aoc(day17, part1, serial)]
pub fn part_1_serial(data: &Arc<Vec<Vec<Block>>>) -> u32 {
    // let mut data = parse(data);
    let end = (data.len() - 1, data[0].len() - 1);
    let mut g_data = GraphData {
        heap: BinaryHeap::new(),
        heap_contents: HashSet::new(),
        min_cost_cache: HashMap::new(),
    };
    let mut down = data[0][0].clone();
    down.cost = 0;
    down.dir = DirCnt::Down(0);
    let mut right = data[0][0].clone();
    right.cost = 0;
    right.dir = DirCnt::Right(0);

    g_data.heap.push(down);
    g_data.heap.push(right);

    while let Some(cur) = g_data.heap.pop() {
        g_data.heap_contents.remove(&(cur.pos, cur.dir));
        if cur.pos == end {
            return cur.cost;
        }
        let cur = if let Some(better_cur) = g_data.min_cost_cache.get(&(cur.pos, cur.dir)) {
            better_cur.clone()
        } else {
            cur
        };
        let neighbors = get_possible_neighbors(&cur, data.len(), data[0].len());
        for new_pos in neighbors {
            if ((cur.dir.is_left() || cur.dir.is_right()) && cur.pos.0 == new_pos.0)
                || ((cur.dir.is_up() || cur.dir.is_down()) && cur.pos.1 == new_pos.1)
            {
                if cur.dir.cnt() + 1 < 3 {
                    same_direction(cur, new_pos, data, &mut g_data);
                }
            } else {
                new_direction(cur, new_pos, data, &mut g_data);
            }
        }
    }
    0
}

#[aoc(day17, part2, serial)]
pub fn part_2_serial(data: &Arc<Vec<Vec<Block>>>) -> u32 {
    // let mut data = parse(data);
    let end = (data.len() - 1, data[0].len() - 1);
    let mut g_data = GraphData {
        heap: BinaryHeap::new(),
        heap_contents: HashSet::new(),
        min_cost_cache: HashMap::new(),
    };
    let mut down = data[0][0].clone();
    down.cost = 0;
    down.dir = DirCnt::Down(0);
    let mut right = data[0][0].clone();
    right.cost = 0;
    right.dir = DirCnt::Right(0);

    g_data.heap.push(down);
    g_data.heap.push(right);

    while let Some(cur) = g_data.heap.pop() {
        g_data.heap_contents.remove(&(cur.pos, cur.dir));
        if cur.pos == end {
            if cur.dir.cnt() >= 3 {
                return cur.cost;
            }
        }
        let cur = if let Some(better_cur) = g_data.min_cost_cache.get(&(cur.pos, cur.dir)) {
            better_cur.clone()
        } else {
            cur
        };
        let neighbors = get_possible_neighbors(&cur, data.len(), data[0].len());
        for new_pos in neighbors {
            if ((cur.dir.is_left() || cur.dir.is_right()) && cur.pos.0 == new_pos.0)
                || ((cur.dir.is_up() || cur.dir.is_down()) && cur.pos.1 == new_pos.1)
            {
                if cur.dir.cnt() + 1 < 10 {
                    same_direction(cur, new_pos, data, &mut g_data);
                }
            } else if cur.dir.cnt() >= 3 {
                new_direction(cur, new_pos, data, &mut g_data);
            }
        }
    }
    0
}

fn am_same_direction(
    cur: Block,
    new_pos: (usize, usize),
    data: &Vec<Vec<Block>>,
    g_data: &RwLock<GraphData>,
) {
    let next_dir = cur.dir.next_dir();
    let mut next = if let Some(next) = g_data
        .read()
        .unwrap()
        .min_cost_cache
        .get(&(new_pos, next_dir))
    {
        next.clone()
    } else {
        data[new_pos.0 as usize][new_pos.1 as usize]
    };
    let temp_cost = cur.cost + next.weight as u32;
    if temp_cost < next.cost {
        next.prev_pos = cur.pos;
        next.cost = temp_cost;
        next.dir = next_dir;
        let mut g_data = g_data.write().unwrap();
        g_data
            .min_cost_cache
            .insert((next.pos, next.dir), next.clone());
        if !g_data.heap_contents.contains(&(next.pos, next.dir)) {
            g_data.heap_contents.insert((next.pos, next.dir));
            g_data.heap.push(next.clone());
        }
    }
}

fn am_new_direction(
    cur: Block,
    new_pos: (usize, usize),
    data: &Vec<Vec<Block>>,
    g_data: &RwLock<GraphData>,
) {
    let next_dir = DirCnt::get_dir(cur.pos, new_pos);
    let mut next = if let Some(next) = g_data
        .read()
        .unwrap()
        .min_cost_cache
        .get(&(new_pos, next_dir))
    {
        next.clone()
    } else {
        data[new_pos.0 as usize][new_pos.1 as usize]
    };
    let temp_cost = cur.cost + next.weight as u32;
    if temp_cost < next.cost {
        next.prev_pos = cur.pos;
        next.cost = temp_cost;
        next.dir = next_dir;
        let mut g_data = g_data.write().unwrap();
        g_data
            .min_cost_cache
            .insert((next.pos, next.dir), next.clone());
        if !g_data.heap_contents.contains(&(next.pos, next.dir)) {
            g_data.heap_contents.insert((next.pos, next.dir));
            g_data.heap.push(next.clone());
        }
    }
}
// This will be a trivial lock based implementation which introduces
// to much overhead to really be worthwhile
// what we really want though is something inspired by
// parallel SSSP algorithms such as the delta-stepping algorithm
// or radius stepping algorithm
#[AmLocalData]
struct Part1 {
    data: Arc<Vec<Vec<Block>>>,
    g_data: Arc<RwLock<GraphData>>,
    end: (usize, usize),
    min: Arc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part1 {
    async fn exec(&self) {
        let pop_heap = |my_data: &RwLock<GraphData>| -> Option<Block> {
            let mut my_data = my_data.write().unwrap();
            if self.min.load(Ordering::SeqCst) == u32::MAX {
                if let Some(x) = my_data.heap.pop() {
                    my_data.heap_contents.remove(&(x.pos, x.dir));
                    Some(x)
                } else {
                    None
                }
            } else {
                None
            }
        };

        while self.min.load(Ordering::SeqCst) == u32::MAX {
            while let Some(cur) = pop_heap(&self.g_data) {
                if cur.pos == self.end {
                    let _ = self.min.compare_exchange(
                        u32::MAX,
                        cur.cost,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    );
                    break;
                }
                let cur = if let Some(better_cur) = self
                    .g_data
                    .read()
                    .unwrap()
                    .min_cost_cache
                    .get(&(cur.pos, cur.dir))
                {
                    better_cur.clone()
                } else {
                    cur
                };
                let neighbors = get_possible_neighbors(&cur, self.data.len(), self.data[0].len());
                for new_pos in neighbors {
                    if ((cur.dir.is_left() || cur.dir.is_right()) && cur.pos.0 == new_pos.0)
                        || ((cur.dir.is_up() || cur.dir.is_down()) && cur.pos.1 == new_pos.1)
                    {
                        if cur.dir.cnt() + 1 < 3 {
                            am_same_direction(cur, new_pos, &self.data, &self.g_data);
                        }
                    } else {
                        am_new_direction(cur, new_pos, &self.data, &self.g_data);
                    }
                }
            }
        }
    }
}

#[aoc(day17, part1, am)]
pub fn part_1_am(data: &Arc<Vec<Vec<Block>>>) -> u32 {
    let end = (data.len() - 1, data[0].len() - 1);
    let mut g_data = GraphData {
        heap: BinaryHeap::new(),
        heap_contents: HashSet::new(),
        min_cost_cache: HashMap::new(),
    };
    let mut down = data[0][0].clone();
    down.cost = 0;
    down.dir = DirCnt::Down(0);
    let mut right = data[0][0].clone();
    right.cost = 0;
    right.dir = DirCnt::Right(0);

    g_data.heap.push(down);
    g_data.heap.push(right);
    let g_data = Arc::new(RwLock::new(g_data));
    let min = Arc::new(AtomicU32::new(u32::MAX));
    for _ in 0..WORLD.num_threads_per_pe() {
        WORLD.exec_am_local(Part1 {
            data: data.clone(),
            g_data: g_data.clone(),
            end: end,
            min: min.clone(),
        });
    }
    WORLD.wait_all();
    min.load(Ordering::SeqCst)
}

#[AmLocalData]
struct Part2 {
    data: Arc<Vec<Vec<Block>>>,
    g_data: Arc<RwLock<GraphData>>,
    end: (usize, usize),
    min: Arc<AtomicU32>,
}

#[local_am]
impl LamellarAm for Part2 {
    async fn exec(&self) {
        let pop_heap = |my_data: &RwLock<GraphData>| -> Option<Block> {
            let mut my_data = my_data.write().unwrap();
            if self.min.load(Ordering::SeqCst) == u32::MAX {
                if let Some(x) = my_data.heap.pop() {
                    my_data.heap_contents.remove(&(x.pos, x.dir));
                    Some(x)
                } else {
                    None
                }
            } else {
                None
            }
        };

        while self.min.load(Ordering::SeqCst) == u32::MAX {
            while let Some(cur) = pop_heap(&self.g_data) {
                if cur.pos == self.end {
                    if cur.dir.cnt() >= 3 {
                        let _ = self.min.compare_exchange(
                            u32::MAX,
                            cur.cost,
                            Ordering::SeqCst,
                            Ordering::SeqCst,
                        );
                        break;
                    }
                }
                let cur = if let Some(better_cur) = self
                    .g_data
                    .read()
                    .unwrap()
                    .min_cost_cache
                    .get(&(cur.pos, cur.dir))
                {
                    better_cur.clone()
                } else {
                    cur
                };
                let neighbors = get_possible_neighbors(&cur, self.data.len(), self.data[0].len());
                for new_pos in neighbors {
                    if ((cur.dir.is_left() || cur.dir.is_right()) && cur.pos.0 == new_pos.0)
                        || ((cur.dir.is_up() || cur.dir.is_down()) && cur.pos.1 == new_pos.1)
                    {
                        if cur.dir.cnt() + 1 < 10 {
                            am_same_direction(cur, new_pos, &self.data, &self.g_data);
                        }
                    } else if cur.dir.cnt() >= 3 {
                        am_new_direction(cur, new_pos, &self.data, &self.g_data);
                    }
                }
            }
        }
    }
}

#[aoc(day17, part2, am)]
pub fn part_2_am(data: &Arc<Vec<Vec<Block>>>) -> u32 {
    let end = (data.len() - 1, data[0].len() - 1);
    let mut g_data = GraphData {
        heap: BinaryHeap::new(),
        heap_contents: HashSet::new(),
        min_cost_cache: HashMap::new(),
    };
    let mut down = data[0][0].clone();
    down.cost = 0;
    down.dir = DirCnt::Down(0);
    let mut right = data[0][0].clone();
    right.cost = 0;
    right.dir = DirCnt::Right(0);

    g_data.heap.push(down);
    g_data.heap.push(right);
    let g_data = Arc::new(RwLock::new(g_data));
    let min = Arc::new(AtomicU32::new(u32::MAX));
    for _ in 0..WORLD.num_threads_per_pe() {
        WORLD.exec_am_local(Part2 {
            data: data.clone(),
            g_data: g_data.clone(),
            end: end,
            min: min.clone(),
        });
    }
    WORLD.wait_all();
    min.load(Ordering::SeqCst)
}
