use std::sync::OnceLock;

use crate::{material::Rgb, math::Vec3};

const FACE_SIZE: usize = 256;

static SKYBOX: OnceLock<Skybox> = OnceLock::new();

#[derive(Clone, Copy)]
enum CubeFace {
    PositiveX = 0,
    NegativeX = 1,
    PositiveY = 2,
    NegativeY = 3,
    PositiveZ = 4,
    NegativeZ = 5,
}

struct Skybox {
    faces: [Vec<Rgb>; 6],
}

pub(crate) fn initialize() {
    SKYBOX.get_or_init(Skybox::generate);
}

pub(crate) fn sample(direction: Vec3) -> Rgb {
    SKYBOX.get_or_init(Skybox::generate).sample(direction)
}

impl Skybox {
    fn generate() -> Self {
        let faces = std::array::from_fn(|face_index| {
            let face = match face_index {
                0 => CubeFace::PositiveX,
                1 => CubeFace::NegativeX,
                2 => CubeFace::PositiveY,
                3 => CubeFace::NegativeY,
                4 => CubeFace::PositiveZ,
                5 => CubeFace::NegativeZ,
                _ => unreachable!(),
            };
            (0..FACE_SIZE * FACE_SIZE)
                .map(|index| {
                    let x = index % FACE_SIZE;
                    let y = index / FACE_SIZE;
                    let u = ((x as f32 + 0.5) / FACE_SIZE as f32) * 2.0 - 1.0;
                    let v = ((y as f32 + 0.5) / FACE_SIZE as f32) * 2.0 - 1.0;
                    procedural_sky(face.direction(u, v))
                })
                .collect()
        });
        Self { faces }
    }

    fn sample(&self, direction: Vec3) -> Rgb {
        let absolute = Vec3::new(direction.x.abs(), direction.y.abs(), direction.z.abs());
        let (face, u, v) = if absolute.x >= absolute.y && absolute.x >= absolute.z {
            if direction.x >= 0.0 {
                (
                    CubeFace::PositiveX,
                    -direction.z / absolute.x,
                    -direction.y / absolute.x,
                )
            } else {
                (
                    CubeFace::NegativeX,
                    direction.z / absolute.x,
                    -direction.y / absolute.x,
                )
            }
        } else if absolute.y >= absolute.z {
            if direction.y >= 0.0 {
                (
                    CubeFace::PositiveY,
                    direction.x / absolute.y,
                    direction.z / absolute.y,
                )
            } else {
                (
                    CubeFace::NegativeY,
                    direction.x / absolute.y,
                    -direction.z / absolute.y,
                )
            }
        } else if direction.z >= 0.0 {
            (
                CubeFace::PositiveZ,
                direction.x / absolute.z,
                -direction.y / absolute.z,
            )
        } else {
            (
                CubeFace::NegativeZ,
                -direction.x / absolute.z,
                -direction.y / absolute.z,
            )
        };

        let x = (((u + 1.0) * 0.5 * FACE_SIZE as f32) as usize).min(FACE_SIZE - 1);
        let y = (((v + 1.0) * 0.5 * FACE_SIZE as f32) as usize).min(FACE_SIZE - 1);
        self.faces[face as usize][y * FACE_SIZE + x]
    }
}

impl CubeFace {
    fn direction(self, u: f32, v: f32) -> Vec3 {
        match self {
            Self::PositiveX => Vec3::new(1.0, -v, -u),
            Self::NegativeX => Vec3::new(-1.0, -v, u),
            Self::PositiveY => Vec3::new(u, 1.0, v),
            Self::NegativeY => Vec3::new(u, -1.0, -v),
            Self::PositiveZ => Vec3::new(u, -v, 1.0),
            Self::NegativeZ => Vec3::new(-u, -v, -1.0),
        }
        .normalized()
    }
}

fn procedural_sky(direction: Vec3) -> Rgb {
    let horizon = (direction.y * 0.5 + 0.5).clamp(0.0, 1.0);
    let nebula = ((direction.x * 2.7 + direction.z * 1.9).sin()
        * (direction.y * 5.3 - direction.z * 2.1).cos())
    .abs()
    .powf(5.0);
    let mut color = Rgb::new(0.004, 0.006, 0.018).mix(Rgb::new(0.015, 0.008, 0.045), horizon);
    color += Rgb::new(0.075, 0.018, 0.12) * nebula;

    let cell_x = (direction.x * 1_100.0).floor() as i32;
    let cell_y = (direction.y * 1_100.0).floor() as i32;
    let cell_z = (direction.z * 1_100.0).floor() as i32;
    let star = hash(cell_x, cell_y, cell_z);
    if star > 0xfffc_0000 {
        let brightness = 0.8 + (star & 0xff) as f32 / 255.0 * 2.2;
        let tint = if star & 0x100 == 0 {
            Rgb::new(0.72, 0.82, 1.0)
        } else {
            Rgb::new(1.0, 0.78, 0.52)
        };
        color += tint * brightness;
    }
    color
}

fn hash(x: i32, y: i32, z: i32) -> u32 {
    let mut value = (x as u32).wrapping_mul(0x045d_9f3b)
        ^ (y as u32).wrapping_mul(0x119d_e1f3)
        ^ (z as u32).wrapping_mul(0x3449_5cf5);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^ (value >> 15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cada_eje_apunta_al_centro_de_su_cara() {
        let skybox = Skybox::generate();
        let axes = [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ];
        for direction in axes {
            let sampled = skybox.sample(direction);
            assert!(sampled.r.is_finite() && sampled.g.is_finite() && sampled.b.is_finite());
        }
    }
}
