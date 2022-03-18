use crate::graph;

use graph::{EdgeEntry, Layout};
use svg::Document;
use svg::node::element::{Path, SVG, Marker, Polygon, Definitions};
use svg::node::element::path::Data;
use svg::node::element::Polyline;

/* Temporary! */
pub fn generate_svg(layout: &Layout) -> SVG {
    const SCALE: i32 = 5;
    let marker = Marker::new()
    .set("id", "arrowhead")
        .set("markerWidth", "10")
        .set("markerHeight", "7")
        .set("refX", "10")
        .set("refY", "3.5")
        .set("orient", "auto")
        .add(Polygon::new()
        .set("points", "0 0, 10 3.5, 0 7"));

    let defs = Definitions::new()
        .add(marker);

    let mut document = Document::new()
        .set("viewBox", (0i32, 0i32, layout.size().0 * SCALE, layout.size().1 * SCALE))
        .add(defs);

    for block in layout.blocks() {
        let data = Data::new()
            .move_to((block.position().0 * SCALE, block.position().1 * SCALE))
            .line_by((0, block.size().1 * SCALE))
            .line_by((block.size().0 * SCALE, 0))
            .line_by((0, -block.size().1 * SCALE))
            .close();
        let path = Path::new()
            .set("fill", "white")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", data);
        document = document.add(path);

        for edge in block.edges() {
            let mut points_str = String::new();
            let mut is_first = true;
            for point in edge.points() {
                if is_first {
                    is_first = false;
                } else {
                    points_str += " ";
                }
                points_str += format!("{},{}", point.0 * (SCALE as f32), point.1 * (SCALE as f32)).as_str();
            }
            let polyline = Polyline::new()
                .set("fill", "none")
                .set("stroke", "blue")
                .set("marker-end", "url(#arrowhead)")
                .set("points", points_str);
            document = document.add(polyline);
        }
    }
    document
}
