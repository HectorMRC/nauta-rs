use std::f64::consts::{FRAC_PI_2, PI};

use super::Shape;
use crate::IntoSvg;
use globe_rs::GeographicPoint;
use svg::node::element::{path::Data, Path};

/// Given a point returns a tuple containing the x and y coordinates to be used in a svg.
fn coordinates(point: GeographicPoint) -> (f64, f64) {
    ((point.longitude() + PI), (FRAC_PI_2 - point.latitude()))
}

impl Into<Path> for Shape {
    fn into(self) -> Path {
        let mut points = self
            .points
            .into_iter()
            .map(GeographicPoint::from)
            .map(coordinates);

        let mut data = Data::new();
        if let Some(position) = points.next() {
            data = data.move_to(position);
        }

        for position in points {
            data = data.line_to(position);
        }

        if self.closed {
            data = data.close();
        }

        Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", data)
    }
}

impl IntoSvg for Shape {
    type Output = Path;
}

#[cfg(test)]
mod test {
    use crate::Shape;
    use globe_rs::CartesianPoint;
    use svg::node::element::Path;

   

    #[test]
    fn svg_path_from_shape() {
        struct TestCase<'a> {
            name: &'a str,
            shape: Shape,
            output: &'a str,
        }

        vec![
            TestCase{
                name: "meridian zero",
                shape: Shape::from(vec![
                    CartesianPoint::new(0., 0., 1.),
                    CartesianPoint::new(0., 0., -1.),
                ]),
                output: "<path d=\"M3.1415927,0 L3.1415927,3.1415927\" fill=\"none\" stroke=\"black\" stroke-width=\"1\"/>"
            },
            TestCase{
                name: "equatorial line",
                shape: Shape::from(vec![
                    CartesianPoint::new(-1., 0., 0.),
                    CartesianPoint::new(-0.9, 0.001445, 0.),
                ]),
                output: "<path d=\"M0,1.5707964 L6.2815927,1.5707964\" fill=\"none\" stroke=\"black\" stroke-width=\"1\"/>"
            }
            
        ].into_iter().for_each(|test_case| {
            let path: Path = test_case.shape.into();
            assert_eq!(path.to_string(), test_case.output, "{}", test_case.name);
        });
    }
}
