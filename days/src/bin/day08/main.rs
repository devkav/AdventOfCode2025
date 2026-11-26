use std::env;

mod part1;
mod part2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let test = args.get(1).unwrap_or(&String::from("")) == "test";
    let input = utils::read_input(8, test);

    println!("Part 1: {}", part1::run(&input, test));
    println!("Part 2: {}", part2::run(&input));
}
