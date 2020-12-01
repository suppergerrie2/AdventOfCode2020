
#[allow(unused_imports)]
use aoc_utils::input_helper::{ask_read_predicate, ask_read, read_vector_from_file, Day};
#[warn(unused_imports)]
use crate::day1::Day1;

mod day1;

fn main() {
    let day = Day1{};

    run_day(&day);
}

fn run_day(day: &dyn Day) {
    day.part1();
    day.part2()
}
