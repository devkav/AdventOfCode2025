mod part1;
mod part2;

fn main() {
    let input = utils::read_input(11);

    println!("Part 1: {}", part1::run(&input));
    println!("Part 2: {}", part2::run(&input));
}
