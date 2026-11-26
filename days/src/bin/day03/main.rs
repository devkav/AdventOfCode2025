use std::env;

mod part1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let test = args[0] == "test";

    let input = utils::read_input(3, test);

    println!("Part 1: {}", part1::run(&input, 2));
    println!("Part 2: {}", part1::run(&input, 12));
}
