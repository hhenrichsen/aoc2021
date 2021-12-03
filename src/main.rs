#![feature(iter_zip)]

use crate::days::day1::{day1_a, day1_b};
use crate::days::day2::{day2_a, day2_b, prep_day2};
use crate::wrapper::{prep_i32, run_day};

mod wrapper;

mod days {
    pub(crate) mod day1;
    pub(crate) mod day2;
}

fn main() {
    run_day("Day 1 A", "res/day1.txt", prep_i32, day1_a, Option::Some(1616));
    run_day("Day 1 B", "res/day1.txt", prep_i32, day1_b, Option::Some(1645));

    run_day("Day 2 A", "res/day2.txt", prep_day2, day2_a, Option::Some(1648020));
    run_day("Day 2 B", "res/day2.txt", prep_day2, day2_b, Option::Some(1759818555));
}
