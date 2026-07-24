use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> f64 {
        let d_sqrd = ((self.x - other.x).pow(2) +
            (self.y - other.y).pow(2) +
            (self.z - other.z).pow(2)) as f64;

        return d_sqrd.sqrt();
    }
}

pub fn run(input: &str) -> usize {
    let mut coordinates: Vec<Coordinate> = Vec::new();
    let mut networks: HashMap<Coordinate, HashSet<Coordinate>> = HashMap::new();

    for line in input.lines() {
        let num_strs: Vec<&str> = line.trim().split(",").collect();
        let nums: Vec<i64> = num_strs.iter().map(|num| num.parse::<i64>().unwrap()).collect();

        if nums.len() != 3 {
            panic!("Expected 3 nums, got {}", nums.len());
        }

        let coordinate = Coordinate {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        };

        let network: HashSet<Coordinate> = HashSet::from([coordinate]);
        coordinates.push(coordinate);
        networks.insert(coordinate, network);
    }

    let num_coordinates = coordinates.len();

    for i in 0..num_coordinates {
        let coordinate = coordinates[i];
        let mut closest: Option<Coordinate> = None;
        let mut closest_distance: Option<f64> = None;

        for j in 0..num_coordinates {
            if i == j {
                continue;
            }
            let other = coordinates[j];
            let current_distance = coordinate.distance(&other);

            if closest.is_none() || current_distance < closest_distance.unwrap() {
                closest = Some(other);
                closest_distance = Some(current_distance);
            }
        }

        let closest = closest.unwrap();
        let closest_network = networks.get(&closest).unwrap();
        let current_network = networks.get(&coordinate).unwrap();

        for network_member in closest_network {
            //current_network.insert(network_member.clone());
        }
    }

    return 0;
}
