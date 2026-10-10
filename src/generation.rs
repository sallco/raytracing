use crate::{
    math::Vec3,
    world::{CelestialBody, GridPosition, Orbit, Voxel},
};

pub(crate) mod material_id {
    pub(crate) const SUN: usize = 0;
    pub(crate) const MERCURY: usize = 1;
    pub(crate) const VENUS: usize = 2;
    pub(crate) const EARTH_OCEAN: usize = 3;
    pub(crate) const EARTH_LAND: usize = 4;
    pub(crate) const MARS: usize = 5;
    pub(crate) const JUPITER: usize = 6;
    pub(crate) const SATURN: usize = 7;
    pub(crate) const SATURN_RING: usize = 8;
    pub(crate) const URANUS: usize = 9;
    pub(crate) const NEPTUNE: usize = 10;
}

#[derive(Clone, Copy)]
struct BodyScale {
    visual_radius: f32,
    detail_radius: f32,
}

impl BodyScale {
    const fn new(visual_radius: f32, detail_radius: f32) -> Self {
        Self {
            visual_radius,
            detail_radius,
        }
    }
}

pub(crate) fn generate_solar_system() -> Vec<CelestialBody> {
    vec![
        body(
            "Sol",
            "Estrella",
            Vec3::ZERO,
            21.0,
            BodyScale::new(6.5, 12.0),
            &[material_id::SUN],
            11,
        )
        .animated(None, 0.10),
        body_at_angle(
            "Mercurio",
            "Planeta rocoso",
            Orbit::new(10.0, 0.25, 0.10),
            BodyScale::new(1.35, 5.0),
            material_id::MERCURY,
            21,
            0.18,
        ),
        body_at_angle(
            "Venus",
            "Planeta rocoso",
            Orbit::new(14.0, 2.55, 0.075),
            BodyScale::new(1.9, 6.0),
            material_id::VENUS,
            22,
            -0.08,
        ),
        body(
            "Tierra",
            "Planeta oceánico",
            orbital_position(18.0, 4.4),
            10.0,
            BodyScale::new(2.1, 7.0),
            &[material_id::EARTH_OCEAN, material_id::EARTH_LAND],
            23,
        )
        .animated(Some(Orbit::new(18.0, 4.4, 0.06)), 0.20),
        body_at_angle(
            "Marte",
            "Planeta rocoso",
            Orbit::new(22.0, 5.55, 0.05),
            BodyScale::new(1.65, 5.5),
            material_id::MARS,
            24,
            0.18,
        ),
        body_at_angle(
            "Júpiter",
            "Gigante gaseoso",
            Orbit::new(30.0, 1.15, 0.035),
            BodyScale::new(4.3, 10.0),
            material_id::JUPITER,
            25,
            0.28,
        ),
        saturn(),
        body_at_angle(
            "Urano",
            "Gigante helado",
            Orbit::new(38.0, 3.55, 0.022),
            BodyScale::new(3.0, 8.0),
            material_id::URANUS,
            27,
            -0.16,
        ),
        body_at_angle(
            "Neptuno",
            "Gigante helado",
            Orbit::new(42.0, 5.0, 0.018),
            BodyScale::new(2.85, 8.0),
            material_id::NEPTUNE,
            28,
            0.15,
        ),
    ]
}

fn body_at_angle(
    name: &'static str,
    kind: &'static str,
    orbit: Orbit,
    scale: BodyScale,
    material: usize,
    seed: u32,
    rotation_speed: f32,
) -> CelestialBody {
    body(
        name,
        kind,
        orbital_position(orbit.radius, orbit.angle),
        scale.visual_radius * 4.0 + 3.0,
        scale,
        &[material],
        seed,
    )
    .animated(Some(orbit), rotation_speed)
}

fn body(
    name: &'static str,
    kind: &'static str,
    center: Vec3,
    focus_distance: f32,
    scale: BodyScale,
    materials: &[usize],
    seed: u32,
) -> CelestialBody {
    CelestialBody::new(
        name,
        kind,
        center,
        focus_distance,
        scale.visual_radius / scale.detail_radius,
        materials[0],
        sphere_surface(scale.detail_radius, materials, seed),
    )
}

fn sphere_surface(radius: f32, materials: &[usize], seed: u32) -> Vec<Voxel> {
    let extent = radius.ceil() as i32;
    let radius_squared = radius * radius;
    let mut voxels = Vec::new();

    for z in -extent..=extent {
        for y in -extent..=extent {
            for x in -extent..=extent {
                let center = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5);
                if center.dot(center) > radius_squared {
                    continue;
                }
                let exposed = [
                    (1, 0, 0),
                    (-1, 0, 0),
                    (0, 1, 0),
                    (0, -1, 0),
                    (0, 0, 1),
                    (0, 0, -1),
                ]
                .into_iter()
                .any(|(dx, dy, dz)| {
                    let neighbor = center + Vec3::new(dx as f32, dy as f32, dz as f32);
                    neighbor.dot(neighbor) > radius_squared
                });
                if exposed {
                    let variation = hash(x, y, z, seed) as usize % materials.len();
                    voxels.push(Voxel {
                        position: GridPosition::new(x, y, z),
                        material: materials[variation],
                    });
                }
            }
        }
    }
    voxels
}

fn saturn() -> CelestialBody {
    const DETAIL_RADIUS: f32 = 9.0;
    const VOXEL_SCALE: f32 = 3.7 / DETAIL_RADIUS;
    let mut voxels = sphere_surface(DETAIL_RADIUS, &[material_id::SATURN], 26);
    for z in -18..=18 {
        for x in -18..=18 {
            let distance = ((x * x + z * z) as f32).sqrt();
            if (11.7..=17.8).contains(&distance) && !hash(x, 0, z, 260).is_multiple_of(7) {
                voxels.push(Voxel {
                    position: GridPosition::new(x, 0, z),
                    material: material_id::SATURN_RING,
                });
            }
        }
    }
    CelestialBody::new(
        "Saturno",
        "Gigante con anillos",
        orbital_position(34.0, 2.35),
        18.5,
        VOXEL_SCALE,
        material_id::SATURN,
        voxels,
    )
    .animated(Some(Orbit::new(34.0, 2.35, 0.028)), 0.24)
}

fn orbital_position(radius: f32, angle: f32) -> Vec3 {
    Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius)
}

fn hash(x: i32, y: i32, z: i32, seed: u32) -> u32 {
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9e37_79b9)
        ^ (y as u32).wrapping_mul(0x85eb_ca6b)
        ^ (z as u32).wrapping_mul(0xc2b2_ae35);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^ (value >> 15)
}
