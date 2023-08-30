use globe_rs::CartesianPoint;
use std::ops::{Index, IndexMut};

pub mod svg;

/// Represents a set of consecutive points.
#[derive(Debug, Clone)]
pub struct Shape {
    points: Vec<CartesianPoint>,
    closed: bool,
}

impl AsRef<[CartesianPoint]> for Shape {
    fn as_ref(&self) -> &[CartesianPoint] {
        &self.points
    }
}

impl AsMut<[CartesianPoint]> for Shape {
    fn as_mut(&mut self) -> &mut [CartesianPoint] {
        &mut self.points
    }
}

impl From<Vec<CartesianPoint>> for Shape {
    fn from(points: Vec<CartesianPoint>) -> Self {
        Self {
            points,
            closed: false,
        }
    }
}

impl Index<usize> for Shape {
    type Output = CartesianPoint;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl IndexMut<usize> for Shape {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.points[index]
    }
}
