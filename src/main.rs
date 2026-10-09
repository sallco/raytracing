#![forbid(unsafe_code)]

use raylib::prelude::*;
use std::time::Instant;

mod camera;
mod generation;
mod geometry;
mod hud;
mod material;
mod math;
mod renderer;
mod scene;
mod sky;
mod world;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 540;

fn main() {
    if std::env::args().any(|argument| argument == "--benchmark") {
        benchmark();
        return;
    }

    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Diorama solar voxelizado")
        .build();

    window.set_target_fps(60);

    let image = Image::gen_image_color(WIDTH, HEIGHT, Color::BLACK);
    let mut framebuffer = match window.load_texture_from_image(&thread, &image) {
        Ok(texture) => texture,
        Err(error) => {
            eprintln!("No fue posible crear el framebuffer: {error}");
            return;
        }
    };
    let scene = scene::Scene::solar_system();
    let mut camera = camera::Camera::overview();
    let mut selection = None;
    let mut dirty = true;
    let mut render_time_ms = 0.0;

    while !window.window_should_close() {
        if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let delta = window.get_mouse_delta();
            if delta.x.abs() > f32::EPSILON || delta.y.abs() > f32::EPSILON {
                camera.orbit(delta.x, delta.y);
                dirty = true;
            }
        }
        let wheel = window.get_mouse_wheel_move();
        if wheel.abs() > f32::EPSILON {
            camera.zoom(wheel);
            dirty = true;
        }
        if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            selection = None;
            camera.reset();
            dirty = true;
        }
        if window.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            selection = Some(selection.map_or(0, |index| (index + 1) % scene.bodies.len()));
            dirty = true;
        }
        if window.is_key_pressed(KeyboardKey::KEY_LEFT) {
            selection = Some(selection.map_or(scene.bodies.len() - 1, |index| {
                (index + scene.bodies.len() - 1) % scene.bodies.len()
            }));
            dirty = true;
        }
        if let Some(index) = selection {
            let body = &scene.bodies[index];
            dirty |= camera.focus(body.center, body.focus_distance);
        }
        if dirty {
            let started = Instant::now();
            let pixels = renderer::render(&scene, camera, WIDTH as usize, HEIGHT as usize);
            render_time_ms = started.elapsed().as_secs_f32() * 1_000.0;
            if let Err(error) = framebuffer.update_texture(&pixels) {
                eprintln!("No fue posible actualizar el framebuffer: {error}");
                break;
            }
            dirty = false;
        }

        let mut drawing = window.begin_drawing(&thread);
        drawing.draw_texture(&framebuffer, 0, 0, Color::WHITE);
        drawing.draw_rectangle(12, 12, 250, 64, Color::new(2, 5, 12, 205));
        drawing.draw_text(
            &format!("{} FPS", drawing.get_fps()),
            24,
            21,
            20,
            Color::new(135, 233, 255, 255),
        );
        drawing.draw_text(
            &format!("Raytrace: {render_time_ms:.1} ms"),
            24,
            47,
            16,
            Color::RAYWHITE,
        );
        hud::draw(&mut drawing, &scene, selection);
    }
}

fn benchmark() {
    let scene = scene::Scene::solar_system();
    let camera = camera::Camera::overview();
    let started = Instant::now();
    let pixels = renderer::render(&scene, camera, WIDTH as usize, HEIGHT as usize);
    std::hint::black_box(pixels);
    let elapsed = started.elapsed().as_secs_f64();
    println!(
        "{} voxeles | {:.1} ms | {:.2} FPS equivalentes",
        scene.total_voxels(),
        elapsed * 1_000.0,
        elapsed.recip()
    );
}
