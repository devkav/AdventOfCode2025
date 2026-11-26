use core::fmt;
use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::coordinate_generic::Coordinate;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Coordinate3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub connections: Vec<usize>,
}

impl fmt::Display for Coordinate3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Coordinate for Coordinate3D {
    fn distance(&self, other: &Coordinate3D) -> f64 {
        let d_sqrd = ((max(self.x, other.x) - min(self.x, other.x)).pow(2)
            + (max(self.y, other.y) - min(self.y, other.y)).pow(2)
            + (max(self.z, other.z) - min(self.z, other.z)).pow(2)) as f64;

        return d_sqrd.sqrt();
    }
}

impl Coordinate3D {
    pub fn volume(&self, other: &Coordinate3D) -> usize {
        let width = max(self.x, other.x) - min(self.x, other.x) + 1;
        let height = max(self.y, other.y) - min(self.y, other.y) + 1;
        let length = max(self.z, other.z) - min(self.z, other.z) + 1;

        println!("{width}, {height}, {length}");

        return width * height * length;
    }

    pub fn graph_contains(&self, other: &Self, coordinates: &Vec<Coordinate3D>) -> bool {
        let mut queue: Vec<&Coordinate3D> = vec![self];
        let mut visited: HashSet<&Coordinate3D> = HashSet::from([self]);

        while queue.len() > 0 {
            let current = queue.pop().unwrap();
            let connections = &current.connections;

            for connection_index in connections {
                let connection = &coordinates[*connection_index];

                if visited.contains(connection) {
                    continue;
                }

                if connection == other {
                    return true;
                }

                visited.insert(connection);
                queue.push(connection);
            }
        }

        return false;
    }

    pub fn add_connection(&mut self, other: usize) {
        self.connections.push(other);
    }
}
