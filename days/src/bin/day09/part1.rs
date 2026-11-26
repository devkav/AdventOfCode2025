use std::cmp::max;
use std::collections::HashMap;
use utils::{coordinate_2d::Coordinate2D, coordinate_generic::Coordinate};

pub fn run(input: &str) -> usize {
    let mut tiles: Vec<Coordinate2D> = Vec::new();
    let mut min_x: Option<usize> = None;
    let mut min_y: Option<usize> = None;
    let mut max_x: Option<usize> = None;
    let mut max_y: Option<usize> = None;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let nums: Vec<usize> = line
            .trim()
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

        tiles.push(Coordinate2D { x, y });
    }

    let min_x = min_x.unwrap();
    let min_y = min_y.unwrap();
    let max_x = max_x.unwrap();
    let max_y = max_y.unwrap();

    let top_left = Coordinate2D { x: min_x, y: min_y };
    let top_right = Coordinate2D { x: max_x, y: min_y };
    let bottom_left = Coordinate2D { x: min_x, y: max_y };
    let bottom_right = Coordinate2D { x: max_x, y: max_y };

    println!("{top_left}");
    println!("{top_right}");
    println!("{bottom_left}");
    println!("{bottom_right}");

    let corners = [top_left, top_right, bottom_left, bottom_right];

    let mut closest_to_corner: HashMap<Coordinate2D, Vec<Coordinate2D>> = HashMap::from([
        (top_left, Vec::new()),
        (top_right, Vec::new()),
        (bottom_left, Vec::new()),
        (bottom_right, Vec::new()),
    ]);

    for tile in tiles {
        for corner in corners {
            let closest = closest_to_corner.get(&corner).unwrap();

            if closest.is_empty() {
                let mut new_arr = closest.clone();
                new_arr.push(tile);
                closest_to_corner.insert(corner, new_arr);
                continue;
            }

            let first = closest.get(0).unwrap();

            if tile.distance(&corner) < first.distance(&corner) {
                let new_arr = vec![tile];
                closest_to_corner.insert(corner, new_arr);
            } else if tile.distance(&corner) == first.distance(&corner) {
                let mut new_arr = closest.clone();
                new_arr.push(tile);
                closest_to_corner.insert(corner, new_arr);
            }
        }
    }

    let mut max_area = 0;

    let top_left_tiles = closest_to_corner.get(&top_left).unwrap();
    let top_right_tiles = closest_to_corner.get(&top_right).unwrap();
    let bottom_left_tiles = closest_to_corner.get(&bottom_left).unwrap();
    let bottom_right_tiles = closest_to_corner.get(&bottom_right).unwrap();

    for top_left_tile in top_left_tiles {
        for bottom_right_tile in bottom_right_tiles {
            max_area = max(max_area, top_left_tile.area(bottom_right_tile));
        }
    }

    for top_right_tile in top_right_tiles {
        for bottom_left_tile in bottom_left_tiles {
            max_area = max(max_area, top_right_tile.area(bottom_left_tile));
        }
    }

    println!("{:?}", closest_to_corner);

    // println!("{area_1}, {area_2}");

    // Find the tiles that are closest to each of the 4 corners.
    // NOTE: There may be multiple tiles that are equally close
    // to a given corner. Include all of these.
    //
    // These corners are defined by combinations of min_x, min_y, max_x and max_y.
    //
    // Next: Find the largest rectangle you can make with some
    // combination of these tiles

    return max_area;
}
