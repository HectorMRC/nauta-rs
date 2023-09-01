use ::svg::{
    node::{element::Group, Attributes},
    Document, Node as SvgNode,
};
use std::f64::consts::PI;

use crate::Karta;

/// Represents any sort of object that can be converted into an [SvgNode].
pub trait IntoSvg: Into<Self::Output>
where
    Self::Output: SvgNode,
{
    type Output;

    fn into_svg(self, attr: Attributes) -> Self::Output;
}

impl<T: IntoSvg> From<Karta<T>> for Document {
    fn from(value: Karta<T>) -> Document {
        let scale = value.scale;
        let mut attributes = Attributes::new();
        attributes.insert("stroke".to_string(), "black".into());
        attributes.insert("stroke-width".to_string(), (1. / scale).into());

        let mut group = Group::new().set("transform", format!("scale({scale})"));
        group.append(value.root.into_traverse().reduce(|value, children| {
            let group = children
                .into_iter()
                .fold(Group::new(), |group, child| group.add(child));

            let mut object = value.into_svg(attributes.clone());
            object.append(group);

            object
        }));

        Document::new()
            .set("width", 2. * PI * scale)
            .set("height", PI * scale)
            .add(group)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use globe_rs::GeographicPoint;
    use svg::Document;

    use crate::{Karta, Shape};

    #[test]
    fn svg_document_from_karta() {
        let karta: Document = Karta::from(Shape::from(vec![
            GeographicPoint::default().with_latitude(FRAC_PI_2),
            GeographicPoint::default().with_latitude(-FRAC_PI_2),
        ]))
        .with_scale(100.)
        .into();

        assert_eq!(karta.to_string(), "<svg height=\"314.1592653589793\" width=\"628.3185307179587\" xmlns=\"http://www.w3.org/2000/svg\">\n<g transform=\"scale(100)\">\n<path d=\"M3.1415927,0 L3.1415927,3.1415927\" stroke=\"black\" stroke-width=\"0.01\">\n<g/>\n</path>\n</g>\n</svg>");
    }
}
