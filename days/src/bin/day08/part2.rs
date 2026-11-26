use std::collections::HashMap;

use utils::{coordinate_3d::Coordinate3D, coordinate_generic::Coordinate};

pub fn run(input: &str) -> usize {
    let mut coordinates: Vec<Coordinate3D> = Vec::new();
    let mut sorted_weights: Vec<(f64, usize, usize)> = Vec::new();
    let mut network_pairings: HashMap<usize, usize> = HashMap::new();
    let mut last_change: (usize, usize) = (0, 0);

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
        network_pairings.insert(i, i);

        for j in i + 1..coordinates.len() {
            let other = &coordinates[j];

            sorted_weights.push((coordinate.distance(other), i, j));
        }
    }

    sorted_weights.sort_by(|a, b| a.0.total_cmp(&b.0));

    println!("Weights have been sorted... Building network...");

    for edge in sorted_weights {
        let a_index = edge.1;
        let b_index = edge.2;
        let a_parent = &network_pairings[&a_index].clone();
        let b_parent = &network_pairings[&b_index];

        if a_parent != b_parent {
            last_change = (a_index, b_index);
            let mut current_network: Vec<usize> = Vec::new();

            for (index, parent) in &network_pairings {
                if parent == b_parent {
                    current_network.push(*index);
                }
            }

            for index in current_network {
                network_pairings.insert(index, *a_parent);
            }
        }
    }

    let last_a = &coordinates[last_change.0];
    let last_b = &coordinates[last_change.1];
    let result = last_a.x * last_b.x;

    return result;
}
