use std::collections::BTreeMap;

use crate::{geometry::Aabb, math::Vec3};

pub(crate) const CHUNK_SIZE: i32 = 8;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct GridPosition {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

impl GridPosition {
    pub(crate) const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn as_vec3(self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    pub(crate) fn from_point(point: Vec3, chunk_extent: f32) -> Self {
        Self::new(
            (point.x / chunk_extent).floor() as i32,
            (point.y / chunk_extent).floor() as i32,
            (point.z / chunk_extent).floor() as i32,
        )
    }

    pub(crate) fn stepped(self, axis: usize, amount: i32) -> Self {
        match axis {
            0 => Self::new(self.x + amount, self.y, self.z),
            1 => Self::new(self.x, self.y + amount, self.z),
            2 => Self::new(self.x, self.y, self.z + amount),
            _ => unreachable!("un eje tridimensional solo puede ser 0, 1 o 2"),
        }
    }

    fn chunk(self) -> Self {
        Self::new(
            self.x.div_euclid(CHUNK_SIZE),
            self.y.div_euclid(CHUNK_SIZE),
            self.z.div_euclid(CHUNK_SIZE),
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Voxel {
    pub(crate) position: GridPosition,
    pub(crate) material: usize,
}

impl Voxel {
    pub(crate) fn bounds(self, scale: f32) -> Aabb {
        let minimum = self.position.as_vec3() * scale;
        Aabb::new(minimum, minimum + Vec3::new(scale, scale, scale))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Chunk {
    pub(crate) bounds: Aabb,
    pub(crate) voxel_indices: Vec<usize>,
}

#[derive(Clone, Debug)]
pub(crate) struct CelestialBody {
    pub(crate) name: &'static str,
    pub(crate) kind: &'static str,
    pub(crate) center: Vec3,
    pub(crate) rotation: f32,
    pub(crate) focus_distance: f32,
    pub(crate) voxel_scale: f32,
    pub(crate) predominant_material: usize,
    pub(crate) bounds: Aabb,
    pub(crate) voxels: Vec<Voxel>,
    pub(crate) chunks: BTreeMap<GridPosition, Chunk>,
}

impl CelestialBody {
    pub(crate) fn new(
        name: &'static str,
        kind: &'static str,
        center: Vec3,
        focus_distance: f32,
        voxel_scale: f32,
        predominant_material: usize,
        voxels: Vec<Voxel>,
    ) -> Self {
        let mut grouped = BTreeMap::<GridPosition, Vec<usize>>::new();
        let mut minimum = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut maximum = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for (index, voxel) in voxels.iter().enumerate() {
            grouped
                .entry(voxel.position.chunk())
                .or_default()
                .push(index);
            let position = voxel.position.as_vec3() * voxel_scale;
            minimum.x = minimum.x.min(position.x);
            minimum.y = minimum.y.min(position.y);
            minimum.z = minimum.z.min(position.z);
            maximum.x = maximum.x.max(position.x + voxel_scale);
            maximum.y = maximum.y.max(position.y + voxel_scale);
            maximum.z = maximum.z.max(position.z + voxel_scale);
        }

        let chunks = grouped
            .into_iter()
            .map(|(position, voxel_indices)| {
                let chunk_extent = CHUNK_SIZE as f32 * voxel_scale;
                let minimum = position.as_vec3() * chunk_extent;
                (
                    position,
                    Chunk {
                        bounds: Aabb::new(
                            minimum,
                            minimum + Vec3::new(chunk_extent, chunk_extent, chunk_extent),
                        ),
                        voxel_indices,
                    },
                )
            })
            .collect();

        Self {
            name,
            kind,
            center,
            rotation: 0.0,
            focus_distance,
            voxel_scale,
            predominant_material,
            bounds: Aabb::new(minimum, maximum),
            voxels,
            chunks,
        }
    }

    pub(crate) fn voxel_count(&self) -> usize {
        self.voxels.len()
    }
}
