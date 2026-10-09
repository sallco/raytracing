use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Rgb {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
}

impl Rgb {
    pub(crate) const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub(crate) const WHITE: Self = Self::new(1.0, 1.0, 1.0);

    pub(crate) const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub(crate) fn mix(self, other: Self, amount: f32) -> Self {
        self * (1.0 - amount) + other * amount
    }

    pub(crate) fn modulate(self, other: Self) -> Self {
        Self::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }

    pub(crate) fn to_rgba8(self) -> [u8; 4] {
        let mapped = Self::new(
            self.r / (1.0 + self.r),
            self.g / (1.0 + self.g),
            self.b / (1.0 + self.b),
        );
        [
            (mapped.r.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8,
            (mapped.g.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8,
            (mapped.b.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8,
            255,
        ]
    }
}

impl Add for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Rgb {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<f32> for Rgb {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Material {
    pub(crate) name: &'static str,
    pub(crate) albedo: Rgb,
    pub(crate) emission: Rgb,
    pub(crate) roughness: f32,
    pub(crate) metallic: f32,
    pub(crate) reflectivity: f32,
    pub(crate) transparency: f32,
    pub(crate) refractive_index: f32,
}

impl Material {
    pub(crate) const fn matte(name: &'static str, albedo: Rgb, roughness: f32) -> Self {
        Self {
            name,
            albedo,
            emission: Rgb::BLACK,
            roughness,
            metallic: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }

    pub(crate) fn emissive(mut self, color: Rgb) -> Self {
        self.emission = color;
        self
    }

    pub(crate) fn reflective(mut self, metallic: f32, reflectivity: f32) -> Self {
        self.metallic = metallic;
        self.reflectivity = reflectivity;
        self
    }

    pub(crate) fn transparent(mut self, transparency: f32, refractive_index: f32) -> Self {
        self.transparency = transparency;
        self.refractive_index = refractive_index;
        self
    }
}
