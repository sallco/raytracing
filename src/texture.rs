use std::sync::OnceLock;

use crate::{material::Rgb, math::Vec3};

const TEXTURE_WIDTH: usize = 256;
const TEXTURE_HEIGHT: usize = 128;
const SURFACE_MAP_SIZE: u32 = 64;

#[derive(Clone, Copy, Debug)]
pub(crate) enum PlanetTexture {
    Sun,
    Mercury,
    Venus,
    EarthOcean,
    EarthLand,
    Mars,
    Jupiter,
    Saturn,
    SaturnRing,
    Uranus,
    Neptune,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum SurfaceTexture {
    Ground104,
    Ground111,
    Metal034,
    Metal040,
    Metal041B,
    Metal041C,
    Metal046B,
    Metal053C,
    Metal056C,
    Metal061B,
    Rock029,
    Rocks011,
    Rocks012,
    Rocks014,
    Rocks024S,
    Rocks025,
}

#[derive(Debug)]
struct SurfaceMap {
    width: usize,
    height: usize,
    albedo: Vec<Rgb>,
    normals: Vec<Vec3>,
    roughness: Vec<f32>,
    metalness: Vec<f32>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TextureSample {
    pub(crate) albedo: Rgb,
    pub(crate) tangent_normal: Vec3,
    pub(crate) roughness: f32,
    pub(crate) metallic: f32,
}

impl SurfaceMap {
    fn load(
        albedo_bytes: &[u8],
        normal_bytes: &[u8],
        roughness_bytes: &[u8],
        metalness_bytes: Option<&[u8]>,
    ) -> Self {
        let image = image::load_from_memory_with_format(albedo_bytes, image::ImageFormat::Png)
            .expect("las texturas PNG embebidas deben ser válidas")
            .resize_exact(
                SURFACE_MAP_SIZE,
                SURFACE_MAP_SIZE,
                image::imageops::FilterType::Triangle,
            )
            .to_rgb8();
        let (width, height) = image.dimensions();
        let albedo: Vec<_> = image
            .pixels()
            .map(|pixel| {
                Rgb::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                )
            })
            .collect();
        let normal_image =
            image::load_from_memory_with_format(normal_bytes, image::ImageFormat::Png)
                .expect("los mapas normales PNG embebidos deben ser válidos")
                .resize_exact(
                    SURFACE_MAP_SIZE,
                    SURFACE_MAP_SIZE,
                    image::imageops::FilterType::Triangle,
                )
                .to_rgb8();
        let normals = normal_image
            .pixels()
            .map(|pixel| {
                let x = pixel[0] as f32 / 127.5 - 1.0;
                let y = pixel[1] as f32 / 127.5 - 1.0;
                let z = pixel[2] as f32 / 127.5 - 1.0;
                Vec3::new(x * 0.42, y * 0.42, z.max(0.05)).normalized()
            })
            .collect();
        let roughness = decode_scalar(roughness_bytes);
        let metalness = metalness_bytes.map_or_else(|| vec![0.0; albedo.len()], decode_scalar);
        debug_assert_eq!(normal_image.dimensions(), (width, height));
        debug_assert_eq!(roughness.len(), albedo.len());
        debug_assert_eq!(metalness.len(), albedo.len());
        Self {
            width: width as usize,
            height: height as usize,
            albedo,
            normals,
            roughness,
            metalness,
        }
    }

    fn sample(&self, uv: [f32; 2]) -> TextureSample {
        let x = (uv[0].rem_euclid(1.0) * self.width as f32) as usize % self.width;
        let y = (uv[1].rem_euclid(1.0) * self.height as f32) as usize % self.height;
        let index = y * self.width + x;
        TextureSample {
            albedo: self.albedo[index],
            tangent_normal: self.normals[index],
            roughness: self.roughness[index],
            metallic: self.metalness[index],
        }
    }
}

fn decode_scalar(bytes: &[u8]) -> Vec<f32> {
    image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .expect("los mapas escalares PNG embebidos deben ser válidos")
        .resize_exact(
            SURFACE_MAP_SIZE,
            SURFACE_MAP_SIZE,
            image::imageops::FilterType::Triangle,
        )
        .to_luma8()
        .pixels()
        .map(|pixel| pixel[0] as f32 / 255.0)
        .collect()
}

impl SurfaceTexture {
    fn map(self) -> &'static SurfaceMap {
        static GROUND104: OnceLock<SurfaceMap> = OnceLock::new();
        static GROUND111: OnceLock<SurfaceMap> = OnceLock::new();
        static METAL034: OnceLock<SurfaceMap> = OnceLock::new();
        static METAL040: OnceLock<SurfaceMap> = OnceLock::new();
        static METAL041B: OnceLock<SurfaceMap> = OnceLock::new();
        static METAL041C: OnceLock<SurfaceMap> = OnceLock::new();
        static METAL046B: OnceLock<SurfaceMap> = OnceLock::new();
        static METAL053C: OnceLock<SurfaceMap> = OnceLock::new();
        static METAL056C: OnceLock<SurfaceMap> = OnceLock::new();
        static METAL061B: OnceLock<SurfaceMap> = OnceLock::new();
        static ROCK029: OnceLock<SurfaceMap> = OnceLock::new();
        static ROCKS011: OnceLock<SurfaceMap> = OnceLock::new();
        static ROCKS012: OnceLock<SurfaceMap> = OnceLock::new();
        static ROCKS014: OnceLock<SurfaceMap> = OnceLock::new();
        static ROCKS024S: OnceLock<SurfaceMap> = OnceLock::new();
        static ROCKS025: OnceLock<SurfaceMap> = OnceLock::new();

        macro_rules! load_surface {
            ($storage:ident, $name:literal) => {
                $storage.get_or_init(|| {
                    SurfaceMap::load(
                        include_bytes!(concat!("../assets/textures/", $name, "_Color.png")),
                        include_bytes!(concat!("../assets/textures/", $name, "_NormalGL.png")),
                        include_bytes!(concat!("../assets/textures/", $name, "_Roughness.png")),
                        None,
                    )
                })
            };
            ($storage:ident, $name:literal, metal) => {
                $storage.get_or_init(|| {
                    SurfaceMap::load(
                        include_bytes!(concat!("../assets/textures/", $name, "_Color.png")),
                        include_bytes!(concat!("../assets/textures/", $name, "_NormalGL.png")),
                        include_bytes!(concat!("../assets/textures/", $name, "_Roughness.png")),
                        Some(include_bytes!(concat!(
                            "../assets/textures/",
                            $name,
                            "_Metalness.png"
                        ))),
                    )
                })
            };
        }

        match self {
            Self::Ground104 => load_surface!(GROUND104, "Ground104"),
            Self::Ground111 => load_surface!(GROUND111, "Ground111"),
            Self::Metal034 => load_surface!(METAL034, "Metal034", metal),
            Self::Metal040 => load_surface!(METAL040, "Metal040", metal),
            Self::Metal041B => load_surface!(METAL041B, "Metal041B", metal),
            Self::Metal041C => load_surface!(METAL041C, "Metal041C", metal),
            Self::Metal046B => load_surface!(METAL046B, "Metal046B", metal),
            Self::Metal053C => load_surface!(METAL053C, "Metal053C", metal),
            Self::Metal056C => load_surface!(METAL056C, "Metal056C", metal),
            Self::Metal061B => load_surface!(METAL061B, "Metal061B", metal),
            Self::Rock029 => load_surface!(ROCK029, "Rock029"),
            Self::Rocks011 => load_surface!(ROCKS011, "Rocks011"),
            Self::Rocks012 => load_surface!(ROCKS012, "Rocks012"),
            Self::Rocks014 => load_surface!(ROCKS014, "Rocks014"),
            Self::Rocks024S => load_surface!(ROCKS024S, "Rocks024S"),
            Self::Rocks025 => load_surface!(ROCKS025, "Rocks025"),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<Rgb>,
    surfaces: Vec<SurfaceTexture>,
    tint_strength: f32,
}

impl Texture {
    pub(crate) fn planet(kind: PlanetTexture, surfaces: &[SurfaceTexture]) -> Self {
        for surface in surfaces {
            surface.map();
        }
        let mut pixels = Vec::with_capacity(TEXTURE_WIDTH * TEXTURE_HEIGHT);
        for y in 0..TEXTURE_HEIGHT {
            let v = (y as f32 + 0.5) / TEXTURE_HEIGHT as f32;
            for x in 0..TEXTURE_WIDTH {
                let u = (x as f32 + 0.5) / TEXTURE_WIDTH as f32;
                pixels.push(planet_color(kind, u, v));
            }
        }
        Self {
            width: TEXTURE_WIDTH,
            height: TEXTURE_HEIGHT,
            pixels,
            surfaces: surfaces.to_vec(),
            tint_strength: planet_tint_strength(kind),
        }
    }

    pub(crate) fn sample(
        &self,
        spherical_uv: [f32; 2],
        texture_uv: [f32; 2],
        variant: u32,
    ) -> TextureSample {
        let u = spherical_uv[0];
        let v = spherical_uv[1];
        let x = (u.rem_euclid(1.0) * self.width as f32) as usize % self.width;
        let y = (v.clamp(0.0, 1.0 - f32::EPSILON) * self.height as f32) as usize;
        let variation = 0.82 + ((variant >> 24) & 0xff) as f32 / 255.0 * 0.30;
        let identity = self.pixels[y * self.width + x];
        let surface = self.surfaces[variant as usize % self.surfaces.len()].map();
        let scale = [0.72, 0.88, 1.04, 1.20][((variant >> 3) & 3) as usize];
        let offset = [
            ((variant >> 8) & 0xff) as f32 / 255.0,
            ((variant >> 16) & 0xff) as f32 / 255.0,
        ];
        let sampled_uv = [
            texture_uv[0] * scale + offset[0],
            texture_uv[1] * scale + offset[1],
        ];
        let mut sample = surface.sample(sampled_uv);
        let identity_luminance = identity.r * 0.2126 + identity.g * 0.7152 + identity.b * 0.0722;
        let tint = identity * (0.55 / identity_luminance.max(0.08));
        let tinted_albedo = sample.albedo.modulate(tint);
        let edge_distance = texture_uv[0]
            .min(1.0 - texture_uv[0])
            .min(texture_uv[1])
            .min(1.0 - texture_uv[1]);
        let edge = 0.88 + 0.12 * (edge_distance / 0.075).clamp(0.0, 1.0);
        sample.albedo = sample.albedo.mix(tinted_albedo, self.tint_strength) * (variation * edge);
        sample
    }
}

fn planet_tint_strength(kind: PlanetTexture) -> f32 {
    match kind {
        PlanetTexture::Sun => 0.12,
        PlanetTexture::Mercury => 0.08,
        PlanetTexture::Venus => 0.15,
        PlanetTexture::EarthOcean => 0.55,
        PlanetTexture::EarthLand => 0.48,
        PlanetTexture::Mars => 0.12,
        PlanetTexture::Jupiter => 0.15,
        PlanetTexture::Saturn => 0.12,
        PlanetTexture::SaturnRing => 0.08,
        PlanetTexture::Uranus => 0.50,
        PlanetTexture::Neptune => 0.58,
    }
}

fn planet_color(kind: PlanetTexture, u: f32, v: f32) -> Rgb {
    let longitude = u * std::f32::consts::TAU;
    let latitude = (0.5 - v) * std::f32::consts::PI;
    let point = [
        latitude.cos() * longitude.cos(),
        latitude.sin(),
        latitude.cos() * longitude.sin(),
    ];
    let detail = fbm(point[0] * 4.0, point[1] * 4.0, point[2] * 4.0);
    let fine = fbm(point[0] * 11.0, point[1] * 11.0, point[2] * 11.0);

    match kind {
        PlanetTexture::Sun => {
            let cells = (detail * 0.7 + fine * 0.3).clamp(0.0, 1.0);
            Rgb::new(1.0, 0.24, 0.015).mix(Rgb::new(1.0, 0.82, 0.18), cells)
        }
        PlanetTexture::Mercury => {
            let craters = if fine < 0.29 { 0.45 } else { 1.0 };
            Rgb::new(0.28, 0.26, 0.24) * ((0.72 + detail * 0.55) * craters)
        }
        PlanetTexture::Venus => {
            let clouds =
                (detail * 0.65 + (longitude * 5.0 + latitude * 2.0).sin() * 0.16).clamp(0.0, 1.0);
            Rgb::new(0.58, 0.25, 0.055).mix(Rgb::new(1.0, 0.79, 0.35), clouds)
        }
        PlanetTexture::EarthOcean => {
            let cloud = (fine > 0.72) as u8 as f32 * 0.45;
            Rgb::new(0.015, 0.08, 0.38)
                .mix(Rgb::new(0.04, 0.36, 0.78), detail)
                .mix(Rgb::WHITE, cloud)
        }
        PlanetTexture::EarthLand => {
            let vegetation = Rgb::new(0.04, 0.27, 0.055)
                .mix(Rgb::new(0.48, 0.34, 0.11), (latitude.abs() * 1.4).min(1.0));
            vegetation.mix(Rgb::new(0.74, 0.69, 0.58), fine * 0.28)
        }
        PlanetTexture::Mars => {
            let canyon = ((longitude * 2.0 + latitude * 0.7).sin().abs() < 0.08) as u8 as f32;
            Rgb::new(0.34, 0.055, 0.018)
                .mix(Rgb::new(0.82, 0.25, 0.075), detail)
                .mix(Rgb::new(0.16, 0.025, 0.012), canyon * 0.55)
        }
        PlanetTexture::Jupiter => {
            let bands = (latitude * 24.0 + detail * 2.3).sin() * 0.5 + 0.5;
            let spot_u = wrapped_distance(u, 0.18) / 0.11;
            let spot_v = (v - 0.61) / 0.055;
            let spot = (spot_u * spot_u + spot_v * spot_v < 1.0) as u8 as f32;
            Rgb::new(0.45, 0.20, 0.09)
                .mix(Rgb::new(0.96, 0.76, 0.48), bands)
                .mix(Rgb::new(0.68, 0.12, 0.045), spot * 0.85)
        }
        PlanetTexture::Saturn => {
            let bands = (latitude * 30.0 + detail).sin() * 0.5 + 0.5;
            Rgb::new(0.52, 0.37, 0.16).mix(Rgb::new(0.96, 0.82, 0.51), bands)
        }
        PlanetTexture::SaturnRing => Rgb::new(0.32, 0.29, 0.24).mix(
            Rgb::new(0.86, 0.79, 0.62),
            (detail * 0.7 + fine * 0.3).powi(2),
        ),
        PlanetTexture::Uranus => {
            let bands = (latitude * 12.0 + detail).sin() * 0.5 + 0.5;
            Rgb::new(0.12, 0.53, 0.58).mix(Rgb::new(0.48, 0.89, 0.88), bands * 0.45)
        }
        PlanetTexture::Neptune => {
            let bands = (latitude * 17.0 + detail * 2.0).sin() * 0.5 + 0.5;
            let storm = (fine < 0.25) as u8 as f32;
            Rgb::new(0.025, 0.09, 0.48)
                .mix(Rgb::new(0.10, 0.38, 0.94), bands)
                .mix(Rgb::new(0.01, 0.025, 0.16), storm * 0.35)
        }
    }
}

fn wrapped_distance(left: f32, right: f32) -> f32 {
    let distance = (left - right).abs();
    distance.min(1.0 - distance)
}

fn fbm(mut x: f32, mut y: f32, mut z: f32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.55;
    for _ in 0..4 {
        value += value_noise(x, y, z) * amplitude;
        x = x * 2.03 + 7.1;
        y = y * 2.03 + 3.7;
        z = z * 2.03 + 5.3;
        amplitude *= 0.48;
    }
    value
}

fn value_noise(x: f32, y: f32, z: f32) -> f32 {
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let iz = z.floor() as i32;
    let fx = smooth(x - x.floor());
    let fy = smooth(y - y.floor());
    let fz = smooth(z - z.floor());

    let mut layers = [0.0; 4];
    for dz in 0..=1 {
        let mut rows = [0.0; 2];
        for dy in 0..=1 {
            let first = hash(ix, iy + dy, iz + dz);
            let second = hash(ix + 1, iy + dy, iz + dz);
            rows[dy as usize] = first + (second - first) * fx;
        }
        layers[dz as usize] = rows[0] + (rows[1] - rows[0]) * fy;
    }
    layers[0] + (layers[1] - layers[0]) * fz
}

fn smooth(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

fn hash(x: i32, y: i32, z: i32) -> f32 {
    let mut value = (x as u32).wrapping_mul(0x9e37_79b9)
        ^ (y as u32).wrapping_mul(0x85eb_ca6b)
        ^ (z as u32).wrapping_mul(0xc2b2_ae35);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    (value ^ (value >> 15)) as f32 / u32::MAX as f32
}
