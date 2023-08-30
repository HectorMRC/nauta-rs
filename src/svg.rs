use svg::Node;

pub trait Svg: Into<Self::Output>
where
    Self::Output: Node,
{
    type Output;
}
