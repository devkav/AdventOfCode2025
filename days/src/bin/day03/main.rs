mod part1;

fn main() {
    let input = utils::read_input(3);

    println!("Part 1: {}", part1::run(&input, 2));
    println!("Part 2: {}", part1::run(&input, 12));
}
