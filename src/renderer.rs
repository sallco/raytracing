use crate::{
    camera::{Camera, Ray},
    geometry::Hit,
    material::Rgb,
    scene::Scene,
    sky,
};

const SURFACE_BIAS: f32 = 0.003;
const MAX_SECONDARY_BOUNCES: u8 = 2;

pub(crate) fn render(scene: &Scene, camera: Camera, width: usize, height: usize) -> Vec<u8> {
    let mut pixels = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let color = trace(scene, camera.ray(x, y, width, height), 0);
            let offset = (y * width + x) * 4;
            pixels[offset..offset + 4].copy_from_slice(&color.to_rgba8());
        }
    }
    pixels
}

fn trace(scene: &Scene, ray: Ray, depth: u8) -> Rgb {
    scene
        .intersect(ray, f32::INFINITY)
        .map_or_else(|| sky::sample(ray.direction), |hit| shade(scene, ray, hit, depth))
}

fn shade(scene: &Scene, ray: Ray, hit: Hit, depth: u8) -> Rgb {
    let material = &scene.materials[hit.material];
    let mut color = material.emission + material.albedo * 0.075;
    let light_vector = scene.light.position - hit.point;
    let light_distance = light_vector.length();
    let light_direction = light_vector / light_distance;
    let shadow_ray = Ray {
        origin: hit.point + hit.normal * SURFACE_BIAS,
        direction: light_direction,
    };

    if !scene.occluded(shadow_ray, light_distance - SURFACE_BIAS) {
        let diffuse = hit.normal.dot(light_direction).max(0.0);
        let attenuation = (scene.light.intensity / (light_distance * light_distance)).min(3.0);
        color += material.albedo.modulate(scene.light.color) * (diffuse * attenuation);

        let view_direction = -ray.direction;
        let halfway = (light_direction + view_direction).normalized();
        let shininess = 2.0 + (1.0 - material.roughness).powi(2) * 126.0;
        let specular = hit.normal.dot(halfway).max(0.0).powf(shininess);
        let specular_color = Rgb::WHITE.mix(material.albedo, material.metallic);
        color += specular_color.modulate(scene.light.color) * (specular * attenuation);
    }

    if material.reflectivity > 0.0 && depth < MAX_SECONDARY_BOUNCES {
        let reflected_direction = ray.direction.reflect(hit.normal).normalized();
        let reflected = trace(
            scene,
            Ray {
                origin: hit.point + hit.normal * SURFACE_BIAS,
                direction: reflected_direction,
            },
            depth + 1,
        );
        let grazing = (1.0 - (-ray.direction).dot(hit.normal).max(0.0)).powi(5);
        let strength = material.reflectivity + (1.0 - material.reflectivity) * grazing;
        color = color.mix(reflected, strength.clamp(0.0, 0.96));
    }

    color
}
