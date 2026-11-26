use core::fmt;
use std::cmp::{max, min};

use crate::coordinate_generic::Coordinate;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coordinate2D {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Coordinate2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coordinate for Coordinate2D {
    fn distance(&self, other: &Coordinate2D) -> f64 {
        let d_sqrd = ((max(self.x, other.x) - min(self.x, other.x)).pow(2)
            + (max(self.y, other.y) - min(self.y, other.y)).pow(2)) as f64;

        return d_sqrd.sqrt();
    }
}

impl Coordinate2D {
    pub fn area(&self, other: &Coordinate2D) -> usize {
        let width = max(self.x, other.x) - min(self.x, other.x) + 1;
        let height = max(self.y, other.y) - min(self.y, other.y) + 1;

        println!("{width}, {height}");

        return width * height;
    }
}
