

use wgpu_renderer::run;

fn main() {
    println!("Hello, world!");

    pollster::block_on(run());
}
