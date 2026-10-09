use crate::{
    camera::Ray,
    generation::{generate_solar_system, material_id},
    geometry::Hit,
    material::{Material, Rgb},
    math::Vec3,
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
                .emissive(Rgb::new(5.2, 2.1, 0.25)),
            Material::matte("Regolito mercuriano", Rgb::new(0.38, 0.34, 0.31), 0.95),
            Material::matte("Nubes de Venus", Rgb::new(0.86, 0.58, 0.23), 0.82),
            Material::matte("Océano terrestre", Rgb::new(0.03, 0.18, 0.62), 0.34),
            Material::matte("Continente terrestre", Rgb::new(0.12, 0.48, 0.16), 0.78),
            Material::matte("Suelo marciano", Rgb::new(0.62, 0.16, 0.06), 0.92),
            Material::matte("Atmósfera de Júpiter", Rgb::new(0.75, 0.47, 0.28), 0.72),
            Material::matte("Atmósfera de Saturno", Rgb::new(0.82, 0.68, 0.39), 0.76),
            Material::matte("Hielo de los anillos", Rgb::new(0.72, 0.68, 0.57), 0.44),
            Material::matte("Hielo de Urano", Rgb::new(0.2, 0.72, 0.76), 0.4),
            Material::matte("Hielo de Neptuno", Rgb::new(0.08, 0.24, 0.83), 0.36),
            Material::matte("Roca de asteroide", Rgb::new(0.32, 0.27, 0.23), 1.0),
            Material::matte("Regolito lunar", Rgb::new(0.46, 0.44, 0.42), 0.96),
            Material::matte("Metal pulido", Rgb::new(0.55, 0.58, 0.62), 0.12).reflective(0.9, 0.72),
            Material::matte("Cristal de hielo", Rgb::new(0.58, 0.82, 0.92), 0.08)
                .transparent(0.82, 1.31),
        ];
        debug_assert_eq!(materials.len() - 1, material_id::GLASS);

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

    pub(crate) fn intersect(&self, ray: Ray, maximum: f32) -> Option<Hit> {
        let mut closest = maximum;
        let mut result = None;

        for body in &self.bodies {
            let local_ray = Ray {
                origin: ray.origin - body.center,
                direction: ray.direction,
            };
            let Some((entry, exit)) = body.bounds.intersect(local_ray, closest) else {
                continue;
            };

            let start = local_ray.at(entry + 0.0001);
            let mut chunk_position = GridPosition::from_point(start);
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
                let boundary = boundary_cell as f32 * CHUNK_SIZE as f32;
                next_crossing[axis] = (boundary - local_ray.origin.component(axis)) / direction;
                crossing_delta[axis] = CHUNK_SIZE as f32 / direction.abs();
            }

            let mut traversal_distance = entry;
            while traversal_distance <= exit && traversal_distance <= closest {
                if let Some(chunk) = body.chunks.get(&chunk_position) {
                    debug_assert!(chunk.bounds.intersect(local_ray, closest).is_some());
                    for &voxel_index in &chunk.voxel_indices {
                        let voxel = body.voxels[voxel_index];
                        let bounds = voxel.bounds();
                        let Some((distance, _)) = bounds.intersect(local_ray, closest) else {
                            continue;
                        };
                        closest = distance;
                        let local_point = local_ray.at(distance);
                        result = Some(Hit {
                            point: local_point + body.center,
                            normal: bounds.normal_at(local_point),
                            material: voxel.material,
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
        self.intersect(ray, maximum).is_some()
    }
}
