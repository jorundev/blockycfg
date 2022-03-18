use crate::graph;

use graph::{EdgeEntry, Layout};
use svg::node::element::path::Data;
use svg::node::element::Polyline;
use svg::node::element::{Definitions, Marker, Path, Polygon, SVG};
use svg::Document;

/* Temporary! */
pub fn generate_svg(layout: &Layout, scale: i32) -> SVG {

    const ARROWIDTH: f32 = 8.0;

    let mut document = Document::new().set(
        "viewBox",
        (0, 0, layout.size().0 * scale, layout.size().1 * scale),
    );

    for block in layout.blocks() {
        let data = Data::new()
            .move_to((block.position().0 * scale, block.position().1 * scale))
            .line_by((0, block.size().1 * scale))
            .line_by((block.size().0 * scale, 0))
            .line_by((0, -block.size().1 * scale))
            .close();
        let path = Path::new()
            .set("fill", "white")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", data);
        document = document.add(path);

        for (i, edge) in block.edges().iter().enumerate(){
            let mut points_str = String::new();

            let fill = match block.edges().len() {
                0 => "none",
                1 => "blue",
                2 => match i {
                    0 => "red",
                    1 => "green",
                    _ => "yellow"
                },
                _ => "yellow"
            };

            let mut polygon = Polygon::new().set("fill", fill);
            for (i, point) in edge.points().iter().enumerate() {
                if i != 0 {
                    points_str += " ";
                }
                let offset = if i == edge.points().len() - 1 {
                    polygon = polygon.set(
                        "points",
                        format!(
                            "{},{} {},{} {},{}",
                            point.0 * scale as f32 - (ARROWIDTH * 0.5),
                            point.1 * scale as f32 - 8f32,
                            point.0 * scale as f32 + (ARROWIDTH * 0.5),
                            point.1 * scale as f32- 8f32,
                            point.0 * scale as f32,
                            point.1 * scale as f32
                        ),
                    );
                    8f32
                } else {
                    0f32
                };
                points_str += format!(
                    "{},{}",
                    point.0 * (scale as f32),
                    point.1 * (scale as f32) - offset
                )
                .as_str();
            }
            let polyline = Polyline::new()
                .set("fill", "none")
                .set("stroke", fill)
                .set("marker-end", "url(#arrowhead)")
                .set("points", points_str);

            document = document.add(polyline).add(polygon);
        }
    }
    document
}
