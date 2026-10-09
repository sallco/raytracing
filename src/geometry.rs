use crate::{camera::Ray, math::Vec3};

const RAY_EPSILON: f32 = 0.001;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Aabb {
    pub(crate) min: Vec3,
    pub(crate) max: Vec3,
}

impl Aabb {
    pub(crate) fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub(crate) fn translated(self, offset: Vec3) -> Self {
        Self::new(self.min + offset, self.max + offset)
    }

    pub(crate) fn intersect(self, ray: Ray, maximum: f32) -> Option<(f32, f32)> {
        let mut near = f32::NEG_INFINITY;
        let mut far = maximum;

        for axis in 0..3 {
            let origin = ray.origin.component(axis);
            let direction = ray.direction.component(axis);
            let minimum = self.min.component(axis);
            let maximum = self.max.component(axis);

            if direction.abs() < f32::EPSILON {
                if origin < minimum || origin > maximum {
                    return None;
                }
                continue;
            }

            let inverse = direction.recip();
            let mut first = (minimum - origin) * inverse;
            let mut second = (maximum - origin) * inverse;
            if first > second {
                std::mem::swap(&mut first, &mut second);
            }
            near = near.max(first);
            far = far.min(second);
            if near > far {
                return None;
            }
        }

        if far < RAY_EPSILON {
            None
        } else {
            Some((if near >= RAY_EPSILON { near } else { far }, far))
        }
    }

    pub(crate) fn normal_at(self, point: Vec3) -> Vec3 {
        let distances = [
            ((point.x - self.min.x).abs(), Vec3::new(-1.0, 0.0, 0.0)),
            ((point.x - self.max.x).abs(), Vec3::new(1.0, 0.0, 0.0)),
            ((point.y - self.min.y).abs(), Vec3::new(0.0, -1.0, 0.0)),
            ((point.y - self.max.y).abs(), Vec3::new(0.0, 1.0, 0.0)),
            ((point.z - self.min.z).abs(), Vec3::new(0.0, 0.0, -1.0)),
            ((point.z - self.max.z).abs(), Vec3::new(0.0, 0.0, 1.0)),
        ];

        distances
            .into_iter()
            .min_by(|left, right| left.0.total_cmp(&right.0))
            .map_or(Vec3::new(0.0, 1.0, 0.0), |(_, normal)| normal)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Hit {
    pub(crate) distance: f32,
    pub(crate) point: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) material: usize,
}
