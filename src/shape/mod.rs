use globe_rs::GeographicPoint;
use std::ops::{Index, IndexMut};

mod svg;
pub use svg::*;

/// Represents a set of consecutive points.
#[derive(Debug, Clone)]
pub struct Shape {
    points: Vec<GeographicPoint>,
    closed: bool,
}

impl AsRef<[GeographicPoint]> for Shape {
    fn as_ref(&self) -> &[GeographicPoint] {
        &self.points
    }
}

impl AsMut<[GeographicPoint]> for Shape {
    fn as_mut(&mut self) -> &mut [GeographicPoint] {
        &mut self.points
    }
}

impl From<Vec<GeographicPoint>> for Shape {
    fn from(points: Vec<GeographicPoint>) -> Self {
        Self {
            points,
            closed: false,
        }
    }
}

impl Index<usize> for Shape {
    type Output = GeographicPoint;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl IndexMut<usize> for Shape {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.points[index]
    }
}
