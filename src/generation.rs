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
    pub(crate) const STATION_HULL: usize = 11;
    pub(crate) const SOLAR_PANEL: usize = 12;
    pub(crate) const SHIP_HULL: usize = 13;
    pub(crate) const ION_ENGINE: usize = 14;
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
    let earth_pos = orbital_position(18.0, 4.4);
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
        .animated(None, 0.06),
        body_at_angle(
            "Mercurio",
            "Planeta rocoso",
            Orbit::new(10.0, 0.25, 0.105),
            BodyScale::new(1.35, 5.0),
            material_id::MERCURY,
            21,
            0.045,
        ),
        body_at_angle(
            "Venus",
            "Planeta rocoso",
            Orbit::new(14.0, 2.55, 0.080),
            BodyScale::new(1.9, 6.0),
            material_id::VENUS,
            22,
            -0.025,
        ),
        body(
            "Tierra",
            "Planeta oceánico",
            earth_pos,
            10.0,
            BodyScale::new(2.1, 7.0),
            &[material_id::EARTH_OCEAN, material_id::EARTH_LAND],
            23,
        )
        .animated(Some(Orbit::new(18.0, 4.4, 0.065)), 0.16),
        space_station(earth_pos),
        satellite(earth_pos),
        body_at_angle(
            "Marte",
            "Planeta rocoso",
            Orbit::new(22.0, 5.55, 0.052),
            BodyScale::new(1.65, 5.5),
            material_id::MARS,
            24,
            0.15,
        ),
        spacecraft(),
        body_at_angle(
            "Júpiter",
            "Gigante gaseoso",
            Orbit::new(30.0, 1.15, 0.032),
            BodyScale::new(4.3, 10.0),
            material_id::JUPITER,
            25,
            0.28,
        ),
        saturn(),
        body_at_angle(
            "Urano",
            "Gigante helado",
            Orbit::new(38.0, 3.55, 0.017),
            BodyScale::new(3.0, 8.0),
            material_id::URANUS,
            27,
            -0.20,
        ),
        body_at_angle(
            "Neptuno",
            "Gigante helado",
            Orbit::new(42.0, 5.0, 0.013),
            BodyScale::new(2.85, 8.0),
            material_id::NEPTUNE,
            28,
            0.21,
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
                    let material =
                        if materials == [material_id::EARTH_OCEAN, material_id::EARTH_LAND] {
                            earth_material(center, x, y, z, seed)
                        } else {
                            let variation = hash(x, y, z, seed) as usize % materials.len();
                            materials[variation]
                        };
                    voxels.push(Voxel {
                        position: GridPosition::new(x, y, z),
                        material,
                    });
                }
            }
        }
    }
    voxels
}

fn earth_material(center: Vec3, x: i32, y: i32, z: i32, seed: u32) -> usize {
    let direction = center.normalized();
    let coastline = (direction.x * 3.7 + direction.z * 1.3).sin() * 0.55
        + (direction.z * 4.1 - direction.y * 1.7).cos() * 0.35
        + ((direction.x + direction.y - direction.z) * 6.0).sin() * 0.20;
    let detail = hash(x, y, z, seed) as f32 / u32::MAX as f32 * 0.16 - 0.08;
    if coastline + detail > 0.16 {
        material_id::EARTH_LAND
    } else {
        material_id::EARTH_OCEAN
    }
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
    .animated(Some(Orbit::new(34.0, 2.35, 0.024)), 0.25)
}

fn space_station(earth_center: Vec3) -> CelestialBody {
    const VOXEL_SCALE: f32 = 0.085;
    const FOCUS_DISTANCE: f32 = 2.8;
    let mut voxels = Vec::new();

    for z in -5..=5 {
        voxels.push(Voxel {
            position: GridPosition::new(0, 0, z),
            material: material_id::STATION_HULL,
        });
        if (-3..=3).contains(&z) {
            voxels.push(Voxel {
                position: GridPosition::new(1, 0, z),
                material: material_id::STATION_HULL,
            });
            voxels.push(Voxel {
                position: GridPosition::new(-1, 0, z),
                material: material_id::STATION_HULL,
            });
            voxels.push(Voxel {
                position: GridPosition::new(0, 1, z),
                material: material_id::STATION_HULL,
            });
            voxels.push(Voxel {
                position: GridPosition::new(0, -1, z),
                material: material_id::STATION_HULL,
            });
        }
    }

    for x in -3..=3 {
        if x != 0 {
            voxels.push(Voxel {
                position: GridPosition::new(x, 0, 0),
                material: material_id::STATION_HULL,
            });
            voxels.push(Voxel {
                position: GridPosition::new(x, 0, 1),
                material: material_id::STATION_HULL,
            });
        }
    }

    for x in -8..=8 {
        voxels.push(Voxel {
            position: GridPosition::new(x, 2, 0),
            material: material_id::STATION_HULL,
        });
    }
    voxels.push(Voxel {
        position: GridPosition::new(0, 1, 0),
        material: material_id::STATION_HULL,
    });

    for &wing_x in &[-8, -7, -6, -5, 5, 6, 7, 8] {
        for z in -3..=3 {
            if z != 0 {
                voxels.push(Voxel {
                    position: GridPosition::new(wing_x, 2, z),
                    material: material_id::SOLAR_PANEL,
                });
            }
        }
    }

    voxels.push(Voxel {
        position: GridPosition::new(0, -2, 0),
        material: material_id::STATION_HULL,
    });

    let orbit = Orbit::secondary(3.9, 1.2, 0.42, 0.28, 3);
    CelestialBody::new(
        "Estación Orbital",
        "Estación espacial",
        earth_center + orbit.offset(),
        FOCUS_DISTANCE,
        VOXEL_SCALE,
        material_id::STATION_HULL,
        voxels,
    )
    .animated(Some(orbit), 0.15)
}

fn satellite(earth_center: Vec3) -> CelestialBody {
    const VOXEL_SCALE: f32 = 0.065;
    const FOCUS_DISTANCE: f32 = 1.9;
    let mut voxels = Vec::new();

    for y in -1..=1 {
        for x in -1..=1 {
            for z in -1..=1 {
                voxels.push(Voxel {
                    position: GridPosition::new(x, y, z),
                    material: material_id::STATION_HULL,
                });
            }
        }
    }

    voxels.push(Voxel {
        position: GridPosition::new(0, 0, 2),
        material: material_id::STATION_HULL,
    });
    for dx in -1..=1 {
        for dy in -1..=1 {
            voxels.push(Voxel {
                position: GridPosition::new(dx, dy, 3),
                material: material_id::STATION_HULL,
            });
        }
    }
    voxels.push(Voxel {
        position: GridPosition::new(0, 0, 4),
        material: material_id::SOLAR_PANEL,
    });

    voxels.push(Voxel {
        position: GridPosition::new(0, 2, 0),
        material: material_id::STATION_HULL,
    });
    voxels.push(Voxel {
        position: GridPosition::new(0, 3, 0),
        material: material_id::STATION_HULL,
    });

    for &wing_x in &[-5, -4, -3, -2, 2, 3, 4, 5] {
        for y in -1..=1 {
            voxels.push(Voxel {
                position: GridPosition::new(wing_x, y, 0),
                material: material_id::SOLAR_PANEL,
            });
        }
    }

    let orbit = Orbit::secondary(4.7, 3.8, -0.35, -0.42, 3);
    CelestialBody::new(
        "Ik'sat",
        "Satélite artificial",
        earth_center + orbit.offset(),
        FOCUS_DISTANCE,
        VOXEL_SCALE,
        material_id::STATION_HULL,
        voxels,
    )
    .animated(Some(orbit), -0.20)
}

fn spacecraft() -> CelestialBody {
    const VOXEL_SCALE: f32 = 0.095;
    const FOCUS_DISTANCE: f32 = 2.7;
    let mut voxels = Vec::new();

    voxels.push(Voxel {
        position: GridPosition::new(0, 0, 6),
        material: material_id::SHIP_HULL,
    });
    voxels.push(Voxel {
        position: GridPosition::new(0, 0, 5),
        material: material_id::SHIP_HULL,
    });

    for y in -1..=1 {
        for x in -1..=1 {
            voxels.push(Voxel {
                position: GridPosition::new(x, y, 4),
                material: material_id::SHIP_HULL,
            });
        }
    }
    voxels.push(Voxel {
        position: GridPosition::new(0, 1, 4),
        material: material_id::SOLAR_PANEL,
    });

    for z in 0..=3 {
        voxels.push(Voxel {
            position: GridPosition::new(0, 0, z),
            material: material_id::SHIP_HULL,
        });
        voxels.push(Voxel {
            position: GridPosition::new(0, 1, z),
            material: material_id::SHIP_HULL,
        });
        for dy in -1..=0 {
            voxels.push(Voxel {
                position: GridPosition::new(-1, dy, z),
                material: material_id::STATION_HULL,
            });
            voxels.push(Voxel {
                position: GridPosition::new(1, dy, z),
                material: material_id::STATION_HULL,
            });
        }
    }

    for &wing_x in &[-3, -2, 2, 3] {
        for z in 1..=3 {
            voxels.push(Voxel {
                position: GridPosition::new(wing_x, 0, z),
                material: material_id::SOLAR_PANEL,
            });
        }
    }

    for z in -2..=-1 {
        for y in -1..=1 {
            for x in -1..=1 {
                voxels.push(Voxel {
                    position: GridPosition::new(x, y, z),
                    material: material_id::SHIP_HULL,
                });
            }
        }
    }

    for &engine_x in &[-1, 1] {
        voxels.push(Voxel {
            position: GridPosition::new(engine_x, 0, -3),
            material: material_id::ION_ENGINE,
        });
        voxels.push(Voxel {
            position: GridPosition::new(engine_x, 1, -3),
            material: material_id::SHIP_HULL,
        });
        voxels.push(Voxel {
            position: GridPosition::new(engine_x, -1, -3),
            material: material_id::SHIP_HULL,
        });
    }

    let orbit = Orbit::independent(20.2, 4.9, 0.058, 0.06);
    CelestialBody::new(
        "Nave Exploradora",
        "Nave espacial",
        orbit.position(),
        FOCUS_DISTANCE,
        VOXEL_SCALE,
        material_id::SHIP_HULL,
        voxels,
    )
    .animated(Some(orbit), 0.18)
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
