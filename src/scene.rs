use crate::{
    camera::Ray,
    generation::{generate_solar_system, material_id},
    geometry::Hit,
    material::{Material, Rgb},
    math::Vec3,
    texture::{PlanetTexture, SurfaceTexture, Texture},
    world::{CHUNK_SIZE, CelestialBody, GridPosition},
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Light {
    pub(crate) position: Vec3,
    pub(crate) color: Rgb,
    pub(crate) intensity: f32,
}

pub(crate) struct Scene {
    pub(crate) materials: Vec<Material>,
    pub(crate) bodies: Vec<CelestialBody>,
    pub(crate) light: Light,
}

impl Scene {
    pub(crate) fn solar_system() -> Self {
        let materials = vec![
            Material::matte("Plasma solar", Rgb::new(1.0, 0.52, 0.08), 0.9)
                .emissive(Rgb::new(5.2, 2.1, 0.25))
                .textured(Texture::planet(
                    PlanetTexture::Sun,
                    &[SurfaceTexture::Metal034],
                )),
            Material::matte("Regolito mercuriano", Rgb::new(0.38, 0.34, 0.31), 0.95).textured(
                Texture::planet(
                    PlanetTexture::Mercury,
                    &[
                        SurfaceTexture::Rocks011,
                        SurfaceTexture::Rocks014,
                        SurfaceTexture::Rocks024S,
                        SurfaceTexture::Rocks025,
                    ],
                ),
            ),
            Material::matte("Nubes de Venus", Rgb::new(0.86, 0.58, 0.23), 0.82).textured(
                Texture::planet(
                    PlanetTexture::Venus,
                    &[
                        SurfaceTexture::Ground104,
                        SurfaceTexture::Metal041C,
                        SurfaceTexture::Metal056C,
                    ],
                ),
            ),
            Material::matte("Océano terrestre", Rgb::new(0.03, 0.18, 0.62), 0.34).textured(
                Texture::planet(
                    PlanetTexture::EarthOcean,
                    &[
                        SurfaceTexture::Metal040,
                        SurfaceTexture::Metal046B,
                        SurfaceTexture::Metal061B,
                    ],
                ),
            ),
            Material::matte("Continente terrestre", Rgb::new(0.12, 0.48, 0.16), 0.78).textured(
                Texture::planet(
                    PlanetTexture::EarthLand,
                    &[
                        SurfaceTexture::Ground111,
                        SurfaceTexture::Rocks012,
                        SurfaceTexture::Rocks025,
                    ],
                ),
            ),
            Material::matte("Suelo marciano", Rgb::new(0.62, 0.16, 0.06), 0.92).textured(
                Texture::planet(
                    PlanetTexture::Mars,
                    &[
                        SurfaceTexture::Ground111,
                        SurfaceTexture::Metal041B,
                        SurfaceTexture::Metal041C,
                        SurfaceTexture::Metal053C,
                        SurfaceTexture::Metal056C,
                        SurfaceTexture::Rock029,
                    ],
                ),
            ),
            Material::matte("Atmósfera de Júpiter", Rgb::new(0.75, 0.47, 0.28), 0.72).textured(
                Texture::planet(
                    PlanetTexture::Jupiter,
                    &[
                        SurfaceTexture::Ground104,
                        SurfaceTexture::Ground111,
                        SurfaceTexture::Rocks012,
                        SurfaceTexture::Rocks025,
                    ],
                ),
            ),
            Material::matte("Atmósfera de Saturno", Rgb::new(0.82, 0.68, 0.39), 0.76).textured(
                Texture::planet(
                    PlanetTexture::Saturn,
                    &[
                        SurfaceTexture::Ground104,
                        SurfaceTexture::Rocks014,
                        SurfaceTexture::Rocks025,
                    ],
                ),
            ),
            Material::matte("Hielo de los anillos", Rgb::new(0.72, 0.68, 0.57), 0.44)
                .reflective(0.1, 0.15)
                .transparent(0.08, 1.31)
                .textured(Texture::planet(
                    PlanetTexture::SaturnRing,
                    &[
                        SurfaceTexture::Rocks011,
                        SurfaceTexture::Rocks012,
                        SurfaceTexture::Rocks014,
                        SurfaceTexture::Rocks024S,
                        SurfaceTexture::Rocks025,
                    ],
                )),
            Material::matte("Hielo de Urano", Rgb::new(0.2, 0.72, 0.76), 0.4).textured(
                Texture::planet(
                    PlanetTexture::Uranus,
                    &[
                        SurfaceTexture::Metal040,
                        SurfaceTexture::Metal046B,
                        SurfaceTexture::Metal061B,
                        SurfaceTexture::Rocks014,
                    ],
                ),
            ),
            Material::matte("Hielo de Neptuno", Rgb::new(0.08, 0.24, 0.83), 0.36).textured(
                Texture::planet(
                    PlanetTexture::Neptune,
                    &[
                        SurfaceTexture::Metal040,
                        SurfaceTexture::Metal046B,
                        SurfaceTexture::Metal061B,
                        SurfaceTexture::Rocks014,
                    ],
                ),
            ),
        ];
        debug_assert_eq!(materials.len() - 1, material_id::NEPTUNE);

        Self {
            materials,
            bodies: generate_solar_system(),
            light: Light {
                position: Vec3::ZERO,
                color: Rgb::new(1.0, 0.72, 0.4),
                intensity: 1_600.0,
            },
        }
    }

    pub(crate) fn total_voxels(&self) -> usize {
        self.bodies.iter().map(CelestialBody::voxel_count).sum()
    }

    pub(crate) fn update(&mut self, delta_seconds: f32) {
        for body in &mut self.bodies {
            body.advance(delta_seconds);
        }
    }

    pub(crate) fn intersect(&self, ray: Ray, maximum: f32) -> Option<Hit> {
        let mut closest = maximum;
        let mut result = None;

        for body in &self.bodies {
            let local_ray = Ray {
                origin: (ray.origin - body.center).rotate_y(-body.rotation),
                direction: ray.direction.rotate_y(-body.rotation),
            };
            let Some((entry, exit)) = body.bounds.intersect(local_ray, closest) else {
                continue;
            };

            let start = local_ray.at(entry + 0.0001);
            let chunk_extent = CHUNK_SIZE as f32 * body.voxel_scale;
            let mut chunk_position = GridPosition::from_point(start, chunk_extent);
            let mut next_crossing = [0.0; 3];
            let mut crossing_delta = [0.0; 3];
            let mut steps = [0; 3];
            for axis in 0..3 {
                let direction = local_ray.direction.component(axis);
                if direction.abs() < f32::EPSILON {
                    next_crossing[axis] = f32::INFINITY;
                    crossing_delta[axis] = f32::INFINITY;
                    continue;
                }
                steps[axis] = if direction > 0.0 { 1 } else { -1 };
                let cell = match axis {
                    0 => chunk_position.x,
                    1 => chunk_position.y,
                    2 => chunk_position.z,
                    _ => unreachable!(),
                };
                let boundary_cell = if steps[axis] > 0 { cell + 1 } else { cell };
                let boundary = boundary_cell as f32 * chunk_extent;
                next_crossing[axis] = (boundary - local_ray.origin.component(axis)) / direction;
                crossing_delta[axis] = chunk_extent / direction.abs();
            }

            let mut traversal_distance = entry;
            while traversal_distance <= exit && traversal_distance <= closest {
                if let Some(chunk) = body.chunks.get(&chunk_position) {
                    debug_assert!(chunk.bounds.intersect(local_ray, closest).is_some());
                    for &voxel_index in &chunk.voxel_indices {
                        let voxel = body.voxels[voxel_index];
                        let bounds = voxel.bounds(body.voxel_scale);
                        let Some((distance, _)) = bounds.intersect(local_ray, closest) else {
                            continue;
                        };
                        closest = distance;
                        let local_point = local_ray.at(distance);
                        let local_normal = bounds.normal_at(local_point);
                        let mapping =
                            cube_mapping(local_point, bounds, local_normal, voxel.position);
                        result = Some(Hit {
                            point: ray.at(distance),
                            normal: local_normal.rotate_y(body.rotation),
                            material: voxel.material,
                            texture_uv: mapping.uv,
                            texture_variant: voxel_texture_variant(voxel.position, voxel.material),
                            tangent: mapping.tangent.rotate_y(body.rotation),
                            bitangent: mapping.bitangent.rotate_y(body.rotation),
                        });
                    }
                }

                let axis = if next_crossing[0] <= next_crossing[1]
                    && next_crossing[0] <= next_crossing[2]
                {
                    0
                } else if next_crossing[1] <= next_crossing[2] {
                    1
                } else {
                    2
                };
                traversal_distance = next_crossing[axis];
                next_crossing[axis] += crossing_delta[axis];
                chunk_position = chunk_position.stepped(axis, steps[axis]);
            }
        }
        result
    }

    pub(crate) fn occluded(&self, ray: Ray, maximum: f32) -> bool {
        self.intersect(ray, maximum).is_some_and(|hit| {
            let emission = self.materials[hit.material].emission;
            emission.r <= f32::EPSILON && emission.g <= f32::EPSILON && emission.b <= f32::EPSILON
        })
    }
}

struct CubeMapping {
    uv: [f32; 2],
    tangent: Vec3,
    bitangent: Vec3,
}

fn cube_mapping(
    point: Vec3,
    bounds: crate::geometry::Aabb,
    normal: Vec3,
    voxel: GridPosition,
) -> CubeMapping {
    let size = bounds.max.x - bounds.min.x;
    let local = (point - bounds.min) / size;
    let (mut uv, mut tangent, mut bitangent) = if normal.x.abs() > 0.5 {
        (
            [
                if normal.x > 0.0 {
                    local.z
                } else {
                    1.0 - local.z
                },
                1.0 - local.y,
            ],
            Vec3::new(0.0, 0.0, normal.x),
            Vec3::new(0.0, -1.0, 0.0),
        )
    } else if normal.y.abs() > 0.5 {
        (
            [
                local.x,
                if normal.y > 0.0 {
                    1.0 - local.z
                } else {
                    local.z
                },
            ],
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -normal.y),
        )
    } else {
        (
            [
                if normal.z > 0.0 {
                    1.0 - local.x
                } else {
                    local.x
                },
                1.0 - local.y,
            ],
            Vec3::new(-normal.z, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
        )
    };

    match voxel_texture_variant(voxel, 0) & 3 {
        0 => {}
        1 => {
            uv = [1.0 - uv[1], uv[0]];
            (tangent, bitangent) = (-bitangent, tangent);
        }
        2 => {
            uv = [1.0 - uv[0], 1.0 - uv[1]];
            (tangent, bitangent) = (-tangent, -bitangent);
        }
        _ => {
            uv = [uv[1], 1.0 - uv[0]];
            (tangent, bitangent) = (bitangent, -tangent);
        }
    }
    CubeMapping {
        uv,
        tangent,
        bitangent,
    }
}

fn voxel_texture_variant(position: GridPosition, material: usize) -> u32 {
    let mut value = (position.x as u32).wrapping_mul(0x9e37_79b9)
        ^ (position.y as u32).wrapping_mul(0x85eb_ca6b)
        ^ (position.z as u32).wrapping_mul(0xc2b2_ae35)
        ^ (material as u32).wrapping_mul(0x27d4_eb2d);
    value ^= value >> 16;
    value.wrapping_mul(0x7feb_352d) ^ (value >> 15)
}
