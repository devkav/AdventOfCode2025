use core::fmt;
use std::collections::HashSet;

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

fn get_num_splits(coord: Coordinate, splitter_columns: &Vec<Vec<Coordinate>>, hit: &mut HashSet<Coordinate>) -> usize {
    let column = &splitter_columns[coord.x];
    let possible_splitters: Vec<&Coordinate> = column.iter().filter(|splitter| splitter.y > coord.y).collect();

    if possible_splitters.len() == 0 {
        return 0;
    } else {
        let hit_splitter = possible_splitters[0];
        
        if hit.contains(hit_splitter) {
            return 0;
        }

        hit.insert(hit_splitter.clone());

        let left = Coordinate { x: hit_splitter.x - 1, y: hit_splitter.y };
        let right = Coordinate { x: hit_splitter.x + 1, y: hit_splitter.y };

        return 1 + get_num_splits(left, &splitter_columns, hit) + get_num_splits(right, &splitter_columns, hit);
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

    let mut hit = HashSet::new();

    return get_num_splits(start_coords, &splitter_columns, &mut hit);
}
