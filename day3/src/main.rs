use aoc_utils::input_helper::{read_string_from_file};
use aoc_utils::day_runner::{run_day};

fn main() {
    run_day(part1, part2)
}

fn part1() {
    let map = load_map();

    let tree_count = check_slope(&(3, 1), &map);

    println!("Amount of trees: {}", tree_count)
}

fn part2() {
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let map = load_map();

    let mut answer = 1;
    for slope in slopes.iter() {
        let tree_count = check_slope(slope, &map);
        println!("{:?} -> {}", slope, tree_count);

        answer *= tree_count;
    }

    println!("Answer is {}", answer)
}

fn load_map() -> Vec<Vec<bool>> {
    let file_contents = read_string_from_file("../input/day3/input.txt").expect("Failed to load file");

    file_contents.split("\r\n").map(|s| s.chars().map(|c| c == '#').collect()).collect()
}

fn check_slope(slope: &(usize, usize), map: &Vec<Vec<bool>>) -> i64 {
    let mut tree_counter: i64 = 0;
    let mut x: usize = 0;
    for y in (0..map.len()).step_by(slope.1) {
        if get_at((x, y), &map) {
            tree_counter += 1;
        }

        x += slope.0;
    }

    tree_counter
}

fn get_at(coord: (usize, usize), map: &Vec<Vec<bool>>) -> bool {
    let r = &map[coord.1];
    r[coord.0 % r.len()]
}