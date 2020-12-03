use aoc_utils::input_helper::{read_vector_from_file};
use aoc_utils::day_runner::run_day;

fn main() {
    run_day(part1, part2)
}

fn part1() {
    let input: Vec<i64>;

    match read_vector_from_file("../input/day1/input.txt", "\r\n") {
        Some(r) => input = r,
        None => panic!("Failed to read vector from file! Did you put the input in input/day1/input.txt?")
    }

    println!("{:?}", input);

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[i] + input[j] == 2020 {
                println!("Output: {} * {} = {}", input[i], input[j], input[i] * input[j])
            }
        }
    }
}

fn part2() {
    let input: Vec<i64>;

    match read_vector_from_file("../input/day1/input.txt", "\r\n") {
        Some(r) => input = r,
        None => panic!("Failed to read vector from file!")
    }

    println!("{:?}", input);

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            for k in (j + 1)..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    println!("Output: {} * {} * {} = {}", input[i], input[j], input[k], input[i] * input[j] * input[k])
                }
            }
        }
    }
}
