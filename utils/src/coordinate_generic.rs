use std::{fmt::Debug, hash::Hash};

pub trait Coordinate: PartialEq + Eq + Hash + Clone +  Debug {
    fn distance(&self, other: &Self) -> f64;
}
