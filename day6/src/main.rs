use aoc_utils::day_runner::run_day;
use std::str::FromStr;
use aoc_utils::input_helper::{read_string_from_file, read_vector_from_file};
use std::collections::HashSet;

fn main() {
    run_day(part1, part2);
}

#[derive(Debug)]
struct Group {
    pub answers: Vec<String>
}

impl Group {
    pub fn count_unique_answers(&self) -> usize {
        let mut a : HashSet<char> = HashSet::new();

        for i in 0..(self.answers.len()) {
            for c in self.answers[i].chars() {
                a.insert(c);
            }
        }

        a.len()
    }

    pub fn count_common_answers(&self) -> usize {
        let mut a : HashSet<char> = self.answers[0].chars().collect();

        for i in 1..(self.answers.len()) {
            a = a.intersection(&self.answers[i].chars().collect()).map(|c| *c).collect();
        }

        a.len()
    }
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a: Vec<String> = s.split("\r\n").map(str::to_string).collect();

        Ok(Group {
            answers: a
        })
    }
}

fn part1() {
    let groups = load_groups();

    let mut total = 0;
    for group in groups {
        let c = group.count_unique_answers();
        total += c;
    }

    println!("Total: {}", total)
}

fn part2() {
    let groups = load_groups();

    let mut total = 0;
    for group in groups {
        let c = group.count_common_answers();
        total += c;
    }

    println!("Total: {}", total)
}

fn load_groups() -> Vec<Group> {
    read_vector_from_file("../input/day6/input.txt", "\r\n\r\n").unwrap()
}