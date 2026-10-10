use std::sync::OnceLock;

use crate::{material::Rgb, math::Vec3};

const SURFACE_MAP_SIZE: u32 = 64;

#[derive(Clone, Copy, Debug)]
pub(crate) enum PlanetTexture {
    Sun,
    Mercury,
    Venus,
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
    Grass005,
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
        static GRASS005: OnceLock<SurfaceMap> = OnceLock::new();
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
            Self::Grass005 => load_surface!(GRASS005, "Grass005"),
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
    surfaces: Vec<SurfaceTexture>,
    tint: Rgb,
    tint_strength: f32,
}

impl Texture {
    pub(crate) fn planet(kind: PlanetTexture, surfaces: &[SurfaceTexture]) -> Self {
        for surface in surfaces {
            surface.map();
        }
        Self {
            surfaces: surfaces.to_vec(),
            tint: planet_tint(kind),
            tint_strength: planet_tint_strength(kind),
        }
    }

    pub(crate) fn sample(&self, texture_uv: [f32; 2], variant: u32) -> TextureSample {
        let variation = 0.82 + ((variant >> 24) & 0xff) as f32 / 255.0 * 0.30;
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
        let tint_luminance = self.tint.r * 0.2126 + self.tint.g * 0.7152 + self.tint.b * 0.0722;
        let normalized_tint = self.tint * (0.55 / tint_luminance.max(0.08));
        let tinted_albedo = sample.albedo.modulate(normalized_tint);
        let edge_distance = texture_uv[0]
            .min(1.0 - texture_uv[0])
            .min(texture_uv[1])
            .min(1.0 - texture_uv[1]);
        let edge = 0.88 + 0.12 * (edge_distance / 0.075).clamp(0.0, 1.0);
        sample.albedo = sample.albedo.mix(tinted_albedo, self.tint_strength) * (variation * edge);
        sample
    }
}

fn planet_tint(kind: PlanetTexture) -> Rgb {
    match kind {
        PlanetTexture::Sun => Rgb::new(1.0, 0.62, 0.12),
        PlanetTexture::Mercury => Rgb::new(0.46, 0.43, 0.40),
        PlanetTexture::Venus => Rgb::new(0.92, 0.63, 0.26),
        PlanetTexture::EarthLand => Rgb::new(0.18, 0.58, 0.20),
        PlanetTexture::Mars => Rgb::new(0.78, 0.24, 0.08),
        PlanetTexture::Jupiter => Rgb::new(0.82, 0.58, 0.38),
        PlanetTexture::Saturn => Rgb::new(0.92, 0.78, 0.48),
        PlanetTexture::SaturnRing => Rgb::new(0.78, 0.73, 0.62),
        PlanetTexture::Uranus => Rgb::new(0.35, 0.82, 0.84),
        PlanetTexture::Neptune => Rgb::new(0.10, 0.30, 0.95),
    }
}

fn planet_tint_strength(kind: PlanetTexture) -> f32 {
    match kind {
        PlanetTexture::Sun => 0.12,
        PlanetTexture::Mercury => 0.08,
        PlanetTexture::Venus => 0.15,
        PlanetTexture::EarthLand => 0.10,
        PlanetTexture::Mars => 0.12,
        PlanetTexture::Jupiter => 0.15,
        PlanetTexture::Saturn => 0.12,
        PlanetTexture::SaturnRing => 0.08,
        PlanetTexture::Uranus => 0.50,
        PlanetTexture::Neptune => 0.58,
    }
}
