mod bsp;
mod app;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let nodes: u32 = args[1].parse().unwrap();
    let width: u32 = args[2].parse().unwrap();
    let height: u32 = args[3].parse().unwrap();

    let bsp = bsp::generate_bsp(nodes, width, height);
    let lines = bsp::bsp_to_lines(&bsp);
    let mut rects = bsp::bsp_to_rectangles(&bsp, width, height);

    app::create_application(400, 400, &lines, &mut rects);
}
