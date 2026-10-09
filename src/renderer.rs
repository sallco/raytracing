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
    let workers = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .clamp(1, 12)
        .min(height);
    let rows_per_worker = height.div_ceil(workers);
    let bytes_per_stripe = rows_per_worker * width * 4;

    std::thread::scope(|scope| {
        for (worker, stripe) in pixels.chunks_mut(bytes_per_stripe).enumerate() {
            let start_y = worker * rows_per_worker;
            scope.spawn(move || {
                for (local_y, row) in stripe.chunks_mut(width * 4).enumerate() {
                    let y = start_y + local_y;
                    for x in 0..width {
                        let color = trace(scene, camera.ray(x, y, width, height), 0);
                        let offset = x * 4;
                        row[offset..offset + 4].copy_from_slice(&color.to_rgba8());
                    }
                }
            });
        }
    });
    pixels
}

fn trace(scene: &Scene, ray: Ray, depth: u8) -> Rgb {
    scene.intersect(ray, f32::INFINITY).map_or_else(
        || sky::sample(ray.direction),
        |hit| shade(scene, ray, hit, depth),
    )
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

    if material.transparency > 0.0
        && depth < MAX_SECONDARY_BOUNCES
        && let Some((direction, outward_normal)) =
            refract(ray.direction, hit.normal, material.refractive_index)
    {
        let transmitted = trace(
            scene,
            Ray {
                origin: hit.point - outward_normal * SURFACE_BIAS,
                direction,
            },
            depth + 1,
        );
        color = color.mix(transmitted, material.transparency);
    }

    color
}

fn refract(
    direction: crate::math::Vec3,
    normal: crate::math::Vec3,
    index: f32,
) -> Option<(crate::math::Vec3, crate::math::Vec3)> {
    let entering = direction.dot(normal) < 0.0;
    let outward_normal = if entering { normal } else { -normal };
    let ratio = if entering { index.recip() } else { index };
    let cosine = (-direction).dot(outward_normal).clamp(0.0, 1.0);
    let discriminant = 1.0 - ratio * ratio * (1.0 - cosine * cosine);
    (discriminant >= 0.0).then(|| {
        (
            (direction * ratio + outward_normal * (ratio * cosine - discriminant.sqrt()))
                .normalized(),
            outward_normal,
        )
    })
}
