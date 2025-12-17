mod part1;

fn main() {
    let part1_result = part1::run(2);
    let part2_result = part1::run(12);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}
