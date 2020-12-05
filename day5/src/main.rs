use aoc_utils::day_runner::run_day;
use aoc_utils::input_helper::{read_string_from_file};
use std::str::Chars;

fn main() {
    run_day(part1, part2)
}

fn part1() {
    let file_contents = read_string_from_file("../input/day5/input.txt").unwrap();
    let passes: Vec<&str> = file_contents.split("\r\n").collect();
    let max = passes.iter().map(calculate_seat_id).max().unwrap();

    println!("Max id is {}", max)
}

fn part2() {
    let file_contents = read_string_from_file("../input/day5/input.txt").unwrap();
    let passes: Vec<&str> = file_contents.split("\r\n").collect();

    let mut seat_ids: Vec<usize> = passes.iter().map(calculate_seat_id).collect();
    seat_ids.sort();

    for i in 0..(seat_ids.len()-1) {
       let id = seat_ids[i];
        if id == seat_ids[i+1]-2 {
            println!("My seat is {}", id+1)
        }
    }
}

fn calculate_seat_id(pass: &&str) -> usize {
    let parts = pass.split_at(7);

    let row = binary_search(0, 127, parts.0.chars());
    let col = binary_search(0, 7, parts.1.chars());

    row * 8 + col
}

fn binary_search(l: usize, u: usize, mut c: Chars) -> usize {
    if u == l {
        return u;
    }

    match c.next() {
        Some('F') => binary_search(l, u - ((u-l)/2 + 1),c),
        Some('B') => binary_search(l + ((u-l)/2+1), u ,c),
        Some('L') => binary_search(l, u - ((u-l)/2 + 1),c),
        Some('R') => binary_search(l + ((u-l)/2+1), u ,c),
        None => u,
        _ => panic!("Unknown character! ")
    }
}