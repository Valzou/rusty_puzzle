mod bsp;
mod app;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let nodes: u32 = args[1].parse().unwrap();
    let width: u32 = args[2].parse().unwrap();
    let height: u32 = args[3].parse().unwrap();

    let (_, mut rects, lines) = bsp::create_game(nodes, width, height);

    app::create_application(width, height, &lines, &mut rects);
}
