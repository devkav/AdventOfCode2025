use crate::coordinate_generic::Coordinate;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Network<T: Coordinate> {
    pub points: Vec<T>,
}

impl<T: Coordinate> Network<T> {
    pub fn size(&self) -> usize {
        return self.points.len();
    }

    pub fn merge(&mut self, other: Network<T>) {
        self.points.extend(other.points);
    }
}
