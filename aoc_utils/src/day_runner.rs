// pub trait Day {
//     fn part1(&self) {
//         eprintln!("part 1 is not implemented")
//     }
//
//     fn part2(&self) {
//         eprintln!("Part 2 is not implemented")
//     }
// }

pub fn run_day<F1, F2>(part1: F1, part2: F2) where F1: Fn(), F2: Fn() {
    println!("Part 1... ");
    // day.part1();
    part1();
    println!("Part 2... ");
    // day.part2()
    part2();
}
