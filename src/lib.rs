mod shape;
use std::f64::consts::PI;

pub use shape::*;

use ::svg::{node::element::Group, Document, Node as SvgNode};
use ntree_rs::Node as TreeNode;

/// Represents any sort of object that can be converted into an [SvgNode].
pub trait IntoSvg: Into<Self::Output>
where
    Self::Output: SvgNode,
{
    type Output;
}

/// Represents a hierarchical group of elements shaping a map.
struct Karta<T> {
    root: TreeNode<T>,
}

impl<T> From<T> for Karta<T> {
    fn from(value: T) -> Self {
        Self {
            root: TreeNode::new(value),
        }
    }
}

impl<T: IntoSvg> Into<Document> for Karta<T> {
    fn into(self) -> Document {
        Document::new().set("width", 2. * PI).set("height", PI).add(
            self.root.into_traverse().reduce(|value, children| {
                let group = children
                    .into_iter()
                    .fold(Group::new(), |group, child| group.add(child));

                let mut object = value.into();
                object.append(group);

                object
            }),
        )
    }
}
