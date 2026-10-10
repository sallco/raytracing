use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Vec3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Vec3 {
    pub(crate) const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub(crate) const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub(crate) fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub(crate) fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub(crate) fn normalized(self) -> Self {
        let length = self.length();
        if length > f32::EPSILON {
            self / length
        } else {
            Self::ZERO
        }
    }

    pub(crate) fn component(self, axis: usize) -> f32 {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => unreachable!("un eje tridimensional solo puede ser 0, 1 o 2"),
        }
    }

    pub(crate) fn reflect(self, normal: Self) -> Self {
        self - normal * (2.0 * self.dot(normal))
    }

    pub(crate) fn rotate_y(self, angle: f32) -> Self {
        let (sine, cosine) = angle.sin_cos();
        Self::new(
            cosine * self.x + sine * self.z,
            self.y,
            -sine * self.x + cosine * self.z,
        )
    }

    pub(crate) fn lerp(self, end: Self, amount: f32) -> Self {
        self + (end - self) * amount
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}
