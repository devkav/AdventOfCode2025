struct Coordinate {
    x: usize,
    y: usize,
}

pub fn run(input: &str) -> usize {
    let mut tiles: Vec<Coordinate> = Vec::new();

    for line in input.lines() {
        let nums: Vec<usize> = line.trim()
            .split(",")
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect();
        let x = nums[0];
        let y = nums[1];

        tiles.push(Coordinate { x, y });
    }

    return 0;
}
