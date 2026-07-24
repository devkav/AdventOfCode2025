struct Coordinate {
    x: usize,
    y: usize,
}

pub fn run(input: &str) -> usize {
    let mut tiles: Vec<Coordinate> = Vec::new();
    let mut min_x: Option<usize> = None;
    let mut min_y: Option<usize> = None;
    let mut max_x: Option<usize> = None;
    let mut max_y: Option<usize> = None;

    for line in input.lines() {
        let nums: Vec<usize> = line.trim()
            .split(",")
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect();
        let x = nums[0];
        let y = nums[1];

        if min_x == None || x < min_x.unwrap() {
            min_x = Some(x);
        }
        if min_y == None || y < min_y.unwrap() {
            min_y = Some(y);
        }
        if max_x == None || x > max_x.unwrap() {
            max_x = Some(x);
        }
        if max_y == None || y > max_y.unwrap() {
            max_y = Some(y);
        }

        tiles.push(Coordinate { x, y });
    }

    let min_x = min_x.unwrap();
    let min_y = min_y.unwrap();
    let max_x = max_x.unwrap();
    let max_y = max_y.unwrap();

    println!("({min_x}, {min_y}) -> ({max_x}, {max_y})");

    // Find the tiles that are closest to each of the 4 corners.
    // NOTE: There may be multiple tiles that are equally close
    // to a given corner. Include all of these.
    //
    // These corners are defined by combinations of min_x, min_y, max_x and max_y.
    //
    // Next: Find the largest rectangle you can make with some
    // combination of these tiles

    return 0;
}
