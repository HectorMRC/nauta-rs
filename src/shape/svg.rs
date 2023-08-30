use super::Shape;
use crate::svg::Svg;
use svg::node::element::Path;

impl Into<Path> for Shape {
    fn into(self) -> Path {
        // let data = Data::new()
        //     .move_to((10, 10))
        //     .line_to((0, 50))
        //     .line_to((50, 0))
        //     .line_to((0, -50))
        //     .close();

        // SvgPath::new()
        //     .set("fill", "none")
        //     .set("stroke", "black")
        //     .set("stroke-width", 3)
        //     .set("d", data)
        todo!()
    }
}

impl Svg for Shape {
    type Output = Path;
}
