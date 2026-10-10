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
        || background(scene, ray),
        |hit| shade(scene, ray, hit, depth),
    )
}

fn background(scene: &Scene, ray: Ray) -> Rgb {
    let mut color = sky::sample(ray.direction);
    color += visible_orbits(scene, ray);
    let to_sun = scene.light.position - ray.origin;
    let projection = to_sun.dot(ray.direction);
    if projection > 0.0 {
        let closest = ray.origin + ray.direction * projection;
        let distance = (closest - scene.light.position).length();
        let halo = 2.6 / (1.0 + (distance / 5.5).powi(4));
        color += Rgb::new(1.0, 0.42, 0.06) * halo;
    }
    color
}

fn visible_orbits(scene: &Scene, ray: Ray) -> Rgb {
    const ORBIT_PLANE_Y: f32 = -0.55;
    const HALF_WIDTH: f32 = 0.22;

    if ray.direction.y.abs() < f32::EPSILON {
        return Rgb::BLACK;
    }
    let distance = (ORBIT_PLANE_Y - ray.origin.y) / ray.direction.y;
    if distance <= 0.0 {
        return Rgb::BLACK;
    }

    let point = ray.at(distance);
    let radius = (point.x * point.x + point.z * point.z).sqrt();
    let proximity = scene
        .bodies
        .iter()
        .filter_map(|body| body.orbit.map(|orbit| (radius - orbit.radius).abs()))
        .fold(f32::INFINITY, f32::min);
    if proximity >= HALF_WIDTH {
        return Rgb::BLACK;
    }

    let strength = (1.0 - proximity / HALF_WIDTH).powi(2);
    Rgb::new(0.09, 0.19, 0.34) * strength
}

fn shade(scene: &Scene, ray: Ray, hit: Hit, depth: u8) -> Rgb {
    let material = &scene.materials[hit.material];
    let surface = material.sample_surface(hit.uv, hit.texture_uv, hit.texture_variant);
    let albedo = surface.albedo;
    let shading_normal = (hit.tangent * surface.tangent_normal.x
        + hit.bitangent * surface.tangent_normal.y
        + hit.normal * surface.tangent_normal.z)
        .normalized();
    let mut color = material.emission.modulate(albedo) + albedo * 0.11;
    let light_vector = scene.light.position - hit.point;
    let light_distance = light_vector.length();
    let light_direction = light_vector / light_distance;
    let shadow_ray = Ray {
        origin: hit.point + hit.normal * SURFACE_BIAS,
        direction: light_direction,
    };

    if !scene.occluded(shadow_ray, light_distance - SURFACE_BIAS) {
        let diffuse = shading_normal.dot(light_direction).max(0.0);
        let attenuation = (scene.light.intensity / (light_distance * light_distance)).min(3.0);
        color += albedo.modulate(scene.light.color) * (diffuse * attenuation);

        let view_direction = -ray.direction;
        let halfway = (light_direction + view_direction).normalized();
        let shininess = 2.0 + (1.0 - surface.roughness).powi(2) * 126.0;
        let specular = shading_normal.dot(halfway).max(0.0).powf(shininess);
        let specular_color = Rgb::WHITE.mix(albedo, surface.metallic);
        color += specular_color.modulate(scene.light.color) * (specular * attenuation);
    }

    if material.reflectivity > 0.0 && depth < MAX_SECONDARY_BOUNCES {
        let reflected_direction = ray.direction.reflect(shading_normal).normalized();
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
