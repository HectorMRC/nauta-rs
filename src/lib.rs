mod shape;
pub use shape::*;

mod svg;
pub use svg::*;

use ntree_rs::Node as TreeNode;

/// Represents a hierarchical group of elements shaping a map.
pub struct Karta<T> {
    root: TreeNode<T>,
    scale: f64,
}

impl<T> From<T> for Karta<T> {
    fn from(value: T) -> Self {
        Self {
            root: TreeNode::new(value),
            scale: 1.,
        }
    }
}

impl<T> Karta<T> {
    /// Sets the scale of the map.
    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }
}
