mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day7;
mod day8;
mod day9;

use lamellar::{LamellarWorld, LamellarWorldBuilder};
use once_cell::sync::Lazy;

static WORLD: Lazy<LamellarWorld> = Lazy::new(|| LamellarWorldBuilder::new().build());
aoc_runner_derive::aoc_lib! { year = 2023 }
