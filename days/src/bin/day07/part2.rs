use core::fmt;
use std::collections::{HashMap};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn get_num_splits(coord: Coordinate, splitter_columns: &Vec<Vec<Coordinate>>, memo: &mut HashMap<Coordinate, usize>) -> usize {
    let column = &splitter_columns[coord.x];
    let possible_splitters: Vec<&Coordinate> = column.iter().filter(|splitter| splitter.y > coord.y).collect();

    if possible_splitters.len() == 0 {
        return 1;
    } else {
        let hit_splitter = possible_splitters[0];

        if memo.contains_key(hit_splitter) {
            return memo[hit_splitter];
        }

        let left = Coordinate { x: hit_splitter.x - 1, y: hit_splitter.y };
        let right = Coordinate { x: hit_splitter.x + 1, y: hit_splitter.y };
        let total = get_num_splits(left, &splitter_columns, memo) + get_num_splits(right, &splitter_columns, memo);

        memo.insert(hit_splitter.clone(), total);

        return total;
    }
}

pub fn run(input: &str) -> usize {
    let mut y = 0;
    let mut start_coords = Coordinate {
        x: 0,
        y: 0
    };
    let mut splitter_columns: Vec<Vec<Coordinate>> = Vec::new();

    for line in input.lines() {
        if splitter_columns.len() == 0 {
            splitter_columns = vec![Vec::new(); line.len()];
        }

        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start_coords = Coordinate { x, y }
            } else if char == '^' {
                splitter_columns[x].push(Coordinate { x, y });
            }
        }

        y += 1;
    }

    for i in 0..splitter_columns.len() {
        splitter_columns[i].sort_by_key(|coord| coord.y);
    }

    let mut memo = HashMap::new();

    return get_num_splits(start_coords, &splitter_columns, &mut memo);
}
