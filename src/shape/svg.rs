use std::f64::consts::{FRAC_PI_2, PI};

use super::Shape;
use crate::IntoSvg;
use globe_rs::GeographicPoint;
use svg::node::{
    element::{path::Data, Path},
    Attributes,
};

/// Given a point returns a tuple containing the x and y coordinates to be used in a svg.
fn as_xy_tuple(point: GeographicPoint) -> (f64, f64) {
    ((point.longitude() + PI), (FRAC_PI_2 - point.latitude()))
}

impl From<Shape> for Path {
    fn from(value: Shape) -> Path {
        let mut points = value.points.into_iter().map(as_xy_tuple);

        let mut data = Data::new();
        if let Some(position) = points.next() {
            data = data.move_to(position);
        }

        for position in points {
            data = data.line_to(position);
        }

        if value.closed {
            data = data.close();
        }

        Path::new().set("d", data)
    }
}

impl IntoSvg for Shape {
    type Output = Path;

    fn into_svg(self, attributes: Attributes) -> Self::Output {
        const NAMES: [&str; 2] = ["stroke", "stroke-width"];

        attributes
            .into_iter()
            .filter(|(name, _)| NAMES.contains(&name.as_str()))
            .fold(self.into(), |path, (name, value)| path.set(name, value))
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::Shape;
    use globe_rs::GeographicPoint;
    use svg::node::element::Path;

    #[test]
    fn svg_path_from_shape() {
        struct TestCase<'a> {
            name: &'a str,
            shape: Shape,
            output: &'a str,
        }

        vec![
            TestCase {
                name: "meridian zero",
                shape: Shape::from(vec![
                    GeographicPoint::default().with_latitude(FRAC_PI_2),
                    GeographicPoint::default().with_latitude(-FRAC_PI_2),
                ]),
                output: "<path d=\"M3.1415927,0 L3.1415927,3.1415927\"/>",
            },
            TestCase {
                name: "equatorial line",
                shape: Shape::from(vec![
                    GeographicPoint::default().with_longitude(-PI).into(),
                    GeographicPoint::default().with_longitude(PI).into(),
                ]),
                output: "<path d=\"M0,1.5707964 L6.2831855,1.5707964\"/>",
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let path: Path = test_case.shape.into();
            assert_eq!(path.to_string(), test_case.output, "{}", test_case.name);
        });
    }
}
