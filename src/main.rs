use blockycfg::graph::EdgeEntry;
use blockycfg::graph::Graph;
use blockycfg::svg::generate_svg;

fn main() {
    let mut graph = Graph::new();
    graph.add_block(0x00, (50, 10), &[EdgeEntry { target: 0x64 }]);
    graph.add_block(0x64, (50, 60), &[]);
    graph.add_block(0x70, (50, 60), &[EdgeEntry { target: 0x64 }]);

    let svg = generate_svg(&graph.layout(0x00), 5);
    svg::save("graph.svg", &svg).unwrap();
}
