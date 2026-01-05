mod camera;
mod color;
mod hittables;
mod material;
mod math;
mod multithreaded_renderer;
mod scenes;
mod util;

use crate::scenes::moving_spheres;

fn main() {
    // part1_final::render();
    moving_spheres::render();
}
