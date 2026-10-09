use crate::math::Vec3;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ray {
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
}

impl Ray {
    pub(crate) fn at(self, distance: f32) -> Vec3 {
        self.origin + self.direction * distance
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Camera {
    pub(crate) target: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
    vertical_fov: f32,
}

impl Camera {
    pub(crate) fn overview() -> Self {
        Self {
            target: Vec3::ZERO,
            yaw: -0.2,
            pitch: 1.12,
            distance: 92.0,
            vertical_fov: 42.0_f32.to_radians(),
        }
    }

    pub(crate) fn position(self) -> Vec3 {
        let horizontal = self.distance * self.pitch.cos();
        self.target
            + Vec3::new(
                horizontal * self.yaw.sin(),
                self.distance * self.pitch.sin(),
                horizontal * self.yaw.cos(),
            )
    }

    pub(crate) fn ray(self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let origin = self.position();
        let forward = (self.target - origin).normalized();
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let right = forward.cross(world_up).normalized();
        let up = right.cross(forward).normalized();
        let aspect = width as f32 / height as f32;
        let scale = (self.vertical_fov * 0.5).tan();
        let screen_x = (2.0 * (x as f32 + 0.5) / width as f32 - 1.0) * aspect * scale;
        let screen_y = (1.0 - 2.0 * (y as f32 + 0.5) / height as f32) * scale;

        Ray {
            origin,
            direction: (forward + right * screen_x + up * screen_y).normalized(),
        }
    }

    pub(crate) fn orbit(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw -= delta_x * 0.006;
        self.pitch = (self.pitch + delta_y * 0.006).clamp(0.25, 1.48);
    }

    pub(crate) fn zoom(&mut self, wheel: f32) {
        self.distance = (self.distance * (1.0 - wheel * 0.1)).clamp(8.0, 150.0);
    }

    pub(crate) fn focus(&mut self, target: Vec3, distance: f32) {
        self.target = self.target.lerp(target, 0.09);
        self.distance += (distance - self.distance) * 0.09;
    }

    pub(crate) fn reset(&mut self) {
        *self = Self::overview();
    }
}
