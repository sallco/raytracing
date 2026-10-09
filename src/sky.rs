use crate::{material::Rgb, math::Vec3};

pub(crate) fn sample(direction: Vec3) -> Rgb {
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
    if star > 0xffff_8000 {
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
    let mut value = (x as u32).wrapping_mul(0x45d9_f3b)
        ^ (y as u32).wrapping_mul(0x119d_e1f3)
        ^ (z as u32).wrapping_mul(0x3449_5cf5);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^ (value >> 15)
}
