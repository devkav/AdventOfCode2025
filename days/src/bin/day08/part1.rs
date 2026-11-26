use std::collections::{HashMap, HashSet};

use utils::{coordinate_3d::Coordinate3D, coordinate_generic::Coordinate};

pub fn run(input: &str, test: bool) -> usize {
    let num_connections = if test { 10 } else { 1000 };

    let mut coordinates: Vec<Coordinate3D> = Vec::new();
    let mut sorted_weights: Vec<(f64, usize, usize)> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let num_strs: Vec<&str> = line.trim().split(",").collect();
        let nums: Vec<usize> = num_strs
            .iter()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        if nums.len() != 3 {
            panic!("Expected 3 nums, got {}", nums.len());
        }

        let coordinate = Coordinate3D {
            x: nums[0],
            y: nums[1],
            z: nums[2],
            connections: vec![],
        };

        coordinates.push(coordinate);
    }

    for i in 0..coordinates.len() {
        let coordinate = &coordinates[i];

        for j in i + 1..coordinates.len() {
            let other = &coordinates[j];

            sorted_weights.push((coordinate.distance(other), i, j));
        }
    }

    sorted_weights.sort_by(|a, b| a.0.total_cmp(&b.0));

    for i in 0..num_connections {
        let edge = sorted_weights[i];
        let a_index = edge.1;
        let b_index = edge.2;
        let a = &coordinates[a_index];
        let b = &coordinates[b_index];
        let same_graph = a.graph_contains(b, &coordinates);

        if !same_graph {
            coordinates[a_index].add_connection(b_index);
            coordinates[b_index].add_connection(a_index);
        }
    }

    let mut network_pairings: HashMap<usize, usize> = HashMap::new();
    let mut network_sizes: HashMap<usize, usize> = HashMap::new();

    for i in 0..coordinates.len() {
        if network_pairings.contains_key(&i) {
            continue;
        }

        let mut queue: Vec<usize> = vec![i];
        let mut visited: HashSet<usize> = HashSet::from([i]);

        while queue.len() > 0 {
            let current_index = queue.pop().unwrap();
            let current = &coordinates[current_index];
            network_pairings.insert(current_index, i);

            for connection_index in &current.connections {
                if visited.contains(connection_index) {
                    continue;
                }

                visited.insert(*connection_index);
                queue.push(*connection_index);
            }
        }

        network_sizes.insert(i, visited.len());
    }

    let mut network_sizes_arr: Vec<usize> = network_sizes.into_values().collect();
    network_sizes_arr.sort();
    network_sizes_arr.reverse();
    let total = network_sizes_arr[..3].iter().product();

    return total;
}
