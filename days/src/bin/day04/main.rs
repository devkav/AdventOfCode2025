use std::env;

mod part1;
mod part2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let test = args[0] == "test";

    let input = utils::read_input(4, test);

    println!("Part 1: {}", part1::run(&input));
    println!("Part 2: {}", part2::run(&input));
}
