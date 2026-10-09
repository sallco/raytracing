use std::collections::BTreeSet;

use crate::{
    math::Vec3,
    world::{CelestialBody, GridPosition, Voxel},
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
    pub(crate) const ASTEROID: usize = 11;
    pub(crate) const REGOLITH: usize = 12;
    pub(crate) const METAL: usize = 13;
    pub(crate) const GLASS: usize = 14;
}

pub(crate) fn generate_solar_system() -> Vec<CelestialBody> {
    let mut bodies = vec![
        body(
            "Sol",
            "Estrella",
            Vec3::ZERO,
            15.0,
            6.5,
            &[material_id::SUN],
            11,
        ),
        body_at_angle(
            "Mercurio",
            "Planeta rocoso",
            10.0,
            0.25,
            1.35,
            material_id::MERCURY,
            21,
        ),
        body_at_angle(
            "Venus",
            "Planeta rocoso",
            14.0,
            2.55,
            1.9,
            material_id::VENUS,
            22,
        ),
        body(
            "Tierra",
            "Planeta oceánico",
            orbital_position(18.0, 4.4),
            6.5,
            2.1,
            &[material_id::EARTH_OCEAN, material_id::EARTH_LAND],
            23,
        ),
        body_at_angle(
            "Marte",
            "Planeta rocoso",
            22.0,
            5.55,
            1.65,
            material_id::MARS,
            24,
        ),
        body_at_angle(
            "Júpiter",
            "Gigante gaseoso",
            30.0,
            1.15,
            4.3,
            material_id::JUPITER,
            25,
        ),
        saturn(),
        body_at_angle(
            "Urano",
            "Gigante helado",
            38.0,
            3.55,
            3.0,
            material_id::URANUS,
            27,
        ),
        body_at_angle(
            "Neptuno",
            "Gigante helado",
            42.0,
            5.0,
            2.85,
            material_id::NEPTUNE,
            28,
        ),
        asteroid_belt(),
    ];
    bodies.push(lunar_terrain(Vec3::new(-43.0, -3.5, 35.0)));
    bodies.push(space_station());
    bodies
}

fn body_at_angle(
    name: &'static str,
    kind: &'static str,
    orbit: f32,
    angle: f32,
    radius: f32,
    material: usize,
    seed: u32,
) -> CelestialBody {
    body(
        name,
        kind,
        orbital_position(orbit, angle),
        radius * 3.0 + 2.0,
        radius,
        &[material],
        seed,
    )
}

fn body(
    name: &'static str,
    kind: &'static str,
    center: Vec3,
    focus_distance: f32,
    radius: f32,
    materials: &[usize],
    seed: u32,
) -> CelestialBody {
    CelestialBody::new(
        name,
        kind,
        center,
        focus_distance,
        materials[0],
        sphere_surface(radius, materials, seed),
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
    let mut voxels = sphere_surface(3.7, &[material_id::SATURN], 26);
    for z in -7..=7 {
        for x in -7..=7 {
            let distance = ((x * x + z * z) as f32).sqrt();
            if (4.8..=7.3).contains(&distance) && !hash(x, 0, z, 260).is_multiple_of(7) {
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
        13.0,
        material_id::SATURN,
        voxels,
    )
}

fn asteroid_belt() -> CelestialBody {
    let mut positions = BTreeSet::new();
    let mut state = 0x5a17_3c91_u32;
    while positions.len() < 3_800 {
        state = xorshift(state);
        let angle = state as f32 / u32::MAX as f32 * std::f32::consts::TAU;
        state = xorshift(state);
        let radius = 23.0 + state as f32 / u32::MAX as f32 * 8.0;
        state = xorshift(state);
        let y = (state % 11) as i32 - 5;
        positions.insert(GridPosition::new(
            (angle.cos() * radius).round() as i32,
            y,
            (angle.sin() * radius).round() as i32,
        ));
    }
    let voxels = positions
        .into_iter()
        .map(|position| Voxel {
            position,
            material: material_id::ASTEROID,
        })
        .collect();
    CelestialBody::new(
        "Cinturón de asteroides",
        "Campo de asteroides",
        Vec3::ZERO,
        55.0,
        material_id::ASTEROID,
        voxels,
    )
}

fn lunar_terrain(center: Vec3) -> CelestialBody {
    let mut voxels = Vec::new();
    for z in -8..8 {
        for x in -8..8 {
            let noise = hash(x, 0, z, 91);
            let height = (noise % 3) as i32 - 1;
            voxels.push(Voxel {
                position: GridPosition::new(x, height, z),
                material: material_id::REGOLITH,
            });
        }
    }
    CelestialBody::new(
        "Superficie lunar",
        "Terreno procedural 16×16",
        center,
        22.0,
        material_id::REGOLITH,
        voxels,
    )
}

fn space_station() -> CelestialBody {
    let mut voxels = Vec::new();
    for z in -5_i32..=5 {
        for x in -8_i32..=8 {
            if x.abs() >= 6 || z.abs() >= 3 || (x + z) % 3 == 0 {
                voxels.push(Voxel {
                    position: GridPosition::new(x, 0, z),
                    material: material_id::METAL,
                });
            }
        }
    }
    for y in 1..=4 {
        for &(x, z) in &[(-5, -2), (-5, 2), (5, -2), (5, 2)] {
            voxels.push(Voxel {
                position: GridPosition::new(x, y, z),
                material: if y == 2 || y == 3 {
                    material_id::GLASS
                } else {
                    material_id::METAL
                },
            });
        }
    }
    CelestialBody::new(
        "Estación Helios",
        "Base espacial",
        Vec3::new(7.0, -3.5, -34.0),
        24.0,
        material_id::METAL,
        voxels,
    )
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

fn xorshift(mut value: u32) -> u32 {
    value ^= value << 13;
    value ^= value >> 17;
    value ^ (value << 5)
}
