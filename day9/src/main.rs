use aoc_utils::day_runner::run_day;
use aoc_utils::input_helper::read_vector_from_file;

fn main() {
    run_day(part1, part2);
}

fn part1() {
    let input: Vec<u64> = read_vector_from_file("../input/day9/input.txt", "\r\n").unwrap();

    println!("{}", find_invalid_number(&input).unwrap())
}

fn part2() {
    let input: Vec<u64> = read_vector_from_file("../input/day9/input.txt", "\r\n").unwrap();

    let invalid_number = find_invalid_number(&input).unwrap();

    println!("Searching for {}", invalid_number);

    let mut top: usize = 0;
    let mut bottom: usize = 0;
    let mut sum: u64 = 0;

    while top < input.len() {

        if sum < invalid_number {
            sum += input[top];
            top += 1;
        } else if sum > invalid_number {
            sum -= input[bottom];
            bottom += 1;
        } else {
            let slice = &input[bottom..top];
            println!("Wow I found it? {:?}", slice);
            let min = slice.iter().min().unwrap();
            let max = slice.iter().max().unwrap();
            println!("Encryption weakness is: {} + {} = {}", min, max, min + max);
            return;
        }

    }
}

fn find_invalid_number(input: &Vec<u64>) -> Option<u64> {
    let preamble_size = if input.len() < 25 { 5 } else { 25 };

    for i in (preamble_size)..(input.len()) {
        let valid = can_sum_to(&input[(i - preamble_size)..i], input[i]);
        if !valid {
            return Some(input[i]);
        }
    }

    None
}

fn can_sum_to(slice: &[u64], sum: u64) -> bool {
    let len = slice.len();

    for i in 0..len {
        for j in (i + 1)..len {
            if slice[i] + slice[j] == sum {
                return true;
            }
        }
    }

    false
}