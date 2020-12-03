use aoc_utils::input_helper::{read_vector_from_file};
use aoc_utils::day_runner::{run_day};

fn main() {
    run_day(part1, part2)
}

fn part1() {
    let input: Vec<String>;

    match read_vector_from_file("../input/day2/input.txt", "\r\n") {
        Some(r) => input = r,
        None => panic!("Failed to read input data, make sure you have an input file at input/day2/input.txt")
    }

    let valid_passwords: Vec<&String> = input.iter().filter(|a| is_valid_password(a, true)).collect();

    println!("Amount of valid passwords: {:?}", valid_passwords.len())
}

fn part2() {
    let input: Vec<String>;

    match read_vector_from_file("../input/day2/input.txt", "\r\n") {
        Some(r) => input = r,
        None => panic!("Failed to read input data, make sure you have an input file at input/day2/input.txt")
    }

    let valid_passwords: Vec<&String> = input.iter().filter(|a| is_valid_password(a, false)).collect();

    println!("Amount of valid passwords: {:?}", valid_passwords.len())
}

fn is_valid_password(string: &&String, part1: bool) -> bool {
    let string_parts: Vec<String> = string.split(": ").map(str::to_string).collect();
    let policy = &string_parts[0];
    let password = &string_parts[1];

    follows_policy(policy, password, part1)
}

fn follows_policy(policy: &String, password: &String, part1: bool) -> bool {
    let policy_parts: Vec<String> = policy.split(" ").map(str::to_string).collect();
    let policy_char: &String = &policy_parts[1];
    let policy_range: (usize, usize) = get_range(&policy_parts[0]);

    if part1 {
        let char_count = password.matches(policy_char).count();

        char_count >= policy_range.0 && policy_range.1 >= char_count
    } else {
        let mut i = 0;
        let pc;

        match policy_char.chars().nth(0) {
            Some(r) => pc = r,
            None => panic!("Could not exctract character from policy char!")
        }

        match password.chars().nth(policy_range.0 - 1) {
            Some(r) => i += if r == pc { 1 } else { 0 },
            None => ()
        }

        match password.chars().nth(policy_range.1 - 1) {
            Some(r) => i += if r == pc { 1 } else { 0 },
            None => ()
        }

        i == 1
    }
}

fn get_range(range_string: &String) -> (usize, usize) {
    let r: Vec<usize> = range_string.split("-").flat_map(str::parse).collect();


    (r[0], r[1])
}