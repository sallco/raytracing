use raylib::prelude::*;

use crate::scene::Scene;

pub(crate) fn draw(
    drawing: &mut RaylibDrawHandle<'_>,
    scene: &Scene,
    selection: Option<usize>,
    width: i32,
    height: i32,
) {
    let panel_y = height - 88;
    let title_y = height - 79;
    let details_y = height - 56;
    let controls_y = height - 40;

    drawing.draw_rectangle(12, panel_y, width - 24, 76, Color::new(2, 5, 12, 215));
    drawing.draw_text(
        "Arrastrar: orbitar  |  Rueda: zoom  |  Flechas Izq/Der: seleccionar  |  Esc: vista general",
        24,
        controls_y,
        16,
        Color::new(170, 184, 205, 255),
    );

    if let Some(index) = selection {
        let body = &scene.bodies[index];
        let material = &scene.materials[body.predominant_material];
        drawing.draw_text(body.name, 24, title_y, 23, Color::new(255, 213, 112, 255));
        drawing.draw_text(
            &format!(
                "{}  |  {} cubos  |  {}",
                body.kind,
                body.voxel_count(),
                material.name
            ),
            24,
            details_y,
            15,
            Color::RAYWHITE,
        );
    } else {
        drawing.draw_text(
            "Sistema Solar voxelizado",
            24,
            title_y,
            23,
            Color::new(255, 213, 112, 255),
        );
        drawing.draw_text(
            &format!(
                "Vista general  |  {} elementos  |  {} cubos visibles",
                scene.bodies.len(),
                scene.total_voxels()
            ),
            24,
            details_y,
            15,
            Color::RAYWHITE,
        );
    }
}
