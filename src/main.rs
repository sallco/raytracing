#![forbid(unsafe_code)]

use raylib::prelude::*;

mod camera;
mod generation;
mod geometry;
mod material;
mod math;
mod scene;
mod sky;
mod world;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 540;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Diorama solar voxelizado")
        .build();

    window.set_target_fps(60);

    while !window.window_should_close() {
        let mut drawing = window.begin_drawing(&thread);
        drawing.clear_background(Color::new(5, 7, 15, 255));
        drawing.draw_text("Raytracer CPU", 20, 20, 24, Color::RAYWHITE);
    }
}
