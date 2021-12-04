#![feature(iter_zip)]

use crate::days::day1::{day1_a, day1_b};
use crate::days::day2::{day2_a, day2_b, prep_day2};
use crate::days::day3::{day3_a, day3_b};
use crate::wrapper::{prep_i32, run_day, run_day_no_prep};

mod wrapper;

mod days {
    pub(crate) mod day1;
    pub(crate) mod day2;
    pub(crate) mod day3;
}

fn main() {
    run_day("Day 1 A", "res/day1.txt", prep_i32, day1_a, Option::Some(1616));
    run_day("Day 1 B", "res/day1.txt", prep_i32, day1_b, Option::Some(1645));

    run_day("Day 2 A", "res/day2.txt", prep_day2, day2_a, Option::Some(1648020));
    run_day("Day 2 B", "res/day2.txt", prep_day2, day2_b, Option::Some(1759818555));

    run_day_no_prep("Day 3 A", "res/day3.txt", day3_a, Option::Some(3009600));
    run_day_no_prep("Day 3 B", "res/day3.txt", day3_b, Option::Some(6940518));

}
