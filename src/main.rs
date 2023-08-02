use glam::Vec3;
use image::{Rgb, RgbImage};

trait ToRgb {
    fn to_rgb(&self) -> Rgb<u8>;
}

impl ToRgb for Vec3 {
    fn to_rgb(&self) -> Rgb<u8> {
        let clamped = self.clamp(Vec3::ZERO, Vec3::ONE);
        Rgb([
            (255.0 * clamped.x) as u8,
            (255.0 * clamped.y) as u8,
            (255.0 * clamped.z) as u8,
        ])
    }
}

#[derive(Copy, Clone)]
struct Light {
    position: Vec3,
    intensity: f32,
}

impl Light {
    fn new(position: Vec3, intensity: f32) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

#[derive(Copy, Clone)]
struct Material {
    diffuse_colour: Vec3,
    albedo: Vec3,
    specular_exponent: f32,
}

impl Material {
    const fn new(diffuse_colour: Vec3, albedo: Vec3, specular_exponent: f32) -> Self {
        Material {
            diffuse_colour,
            albedo,
            specular_exponent,
        }
    }

    const IVORY: Self = Self::new(
        Vec3 {
            x: 0.4,
            y: 0.4,
            z: 0.3,
        },
        Vec3 {
            x: 0.6,
            y: 0.3,
            z: 0.3,
        },
        50.0,
    );

    const RED_RUBBER: Self = Self::new(
        Vec3 {
            x: 0.3,
            y: 0.1,
            z: 0.1,
        },
        Vec3 {
            x: 0.9,
            y: 0.1,
            z: 0.1,
        },
        10.0,
    );

    const MIRROR: Self = Self::new(
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Vec3 {
            x: 0.0,
            y: 10.0,
            z: 0.8,
        },
        1425.0,
    );
}

impl Default for Material {
    fn default() -> Self {
        Material {
            diffuse_colour: (0.2, 0.7, 0.8).into(),
            albedo: (1.0, 0.0, 0.0).into(),
            specular_exponent: 0.0,
        }
    }
}

struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Option<f32> {
        // Produce a vector that is from the originating point
        // to the centre of the sphere
        let delta = self.center - origin.clone();

        // See if sphere is in the front of at behind of this direction.
        let projection = delta.dot(*direction);

        // Aright triangle formed by delta, projection and the orthogonal to
        // the projection. Check if that third distance is less than radius
        let orth_sq = delta.dot(delta) - projection * projection;
        let radius_sq = self.radius * self.radius;

        // Now calculate the size of the intersected bit
        let half_segment = f32::sqrt(radius_sq - orth_sq);

        if orth_sq > radius_sq {
            return None;
        }

        let first_intersection_distance_closer = projection - half_segment;
        let first_intersection_distance_farther = delta.dot(*direction) + half_segment;

        if first_intersection_distance_closer < 0.0 {
            if first_intersection_distance_farther < 0.0 {
                None
            } else {
                Some(first_intersection_distance_farther)
            }
        } else {
            Some(first_intersection_distance_closer)
        }
    }
}

fn cast_ray(
    origin: &Vec3,
    direction: &Vec3,
    spheres: &Vec<Sphere>,
    lights: &Vec<Light>,
    depth: usize,
) -> Vec3 {
    let bg_colour = Material::default().diffuse_colour;
    if depth > 4 {
        return bg_colour;
    }
    match scene_intersect(origin, direction, spheres) {
        Some((material, point, normal)) => {
            let reflect_dir = reflect(direction, &normal).normalize();
            let reflect_origin = if reflect_dir.dot(normal) < 0.0 {
                point - normal * 1e-3
            } else {
                point + normal * 1e-3
            };
            let reflect_colour =
                cast_ray(&reflect_origin, &reflect_dir, spheres, lights, depth + 1);

            let mut diffuse_light_intensity: f32 = 0.0;
            let mut specular_light_intensity: f32 = 0.0;

            for light in lights {
                let light_dir = (light.position - point).normalize();
                let light_distance = (light.position - point).length();

                let shadow_origin = if light_dir.dot(normal) < 0.0 {
                    point - normal * 1e-4
                } else {
                    point + normal * 1e-4
                };

                if let Some((_tmpmaterial, shadow_point, _shadow_normal)) =
                    scene_intersect(&shadow_origin, &light_dir, spheres)
                {
                    if (shadow_point - shadow_origin).length() < light_distance {
                        continue;
                    }
                }

                diffuse_light_intensity += light.intensity * f32::max(0.0, light_dir.dot(normal));
                let reflection = reflect(&light_dir, &normal);
                specular_light_intensity += f32::powf(
                    f32::max(0.0, reflection.dot(*direction)),
                    material.specular_exponent,
                ) * light.intensity;
            }

            return material.diffuse_colour * diffuse_light_intensity * material.albedo.x
                + Vec3::ONE * specular_light_intensity * material.albedo.y
                + reflect_colour * material.albedo.z;
        }
        None => bg_colour,
    }
}

fn reflect(inverse: &Vec3, normal: &Vec3) -> Vec3 {
    let inverse_dot_norm = inverse.dot(*normal);
    *inverse - 2.0 * (*normal) * inverse_dot_norm
}

fn scene_intersect(
    origin: &Vec3,
    direction: &Vec3,
    spheres: &Vec<Sphere>,
) -> Option<(Material, Vec3, Vec3)> {
    let mut spheres_dist = std::f32::MAX;
    let mut hit = Vec3::default();
    let mut normal = Vec3::default();
    let mut material = Material::default();
    for sphere in spheres {
        if let Some(dist_i) = sphere.ray_intersect(origin, direction) {
            spheres_dist = dist_i;
            hit = *origin + (*direction) * dist_i;
            normal = (hit - sphere.center).normalize();
            material = sphere.material;
        }
    }
    if spheres_dist < 1000.0 {
        return Some((material, hit, normal));
    }
    None
}

fn render(
    spheres: &Vec<Sphere>,
    lights: &Vec<Light>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let width: u32 = 1024; // px
    let height: u32 = 768; // px
    let field_of_view: f32 = std::f32::consts::FRAC_PI_2;

    let origin: Vec3 = (0.0, 0.0, 0.0).into();

    let width_f: f32 = width as f32;
    let height_f: f32 = height as f32;

    let mut framebuffer = RgbImage::new(width, height);

    for j in 0..height {
        for i in 0..width {
            let x =
                (2.0 * (i as f32 + 0.5) / width_f - 1.0) * f32::tan(field_of_view / 2.0) * width_f
                    / height_f;
            let y = -(2.0 * (j as f32 + 0.5) / height_f - 1.0) * f32::tan(field_of_view / 2.0);

            let direction = Vec3::new(x, y, -1.0).normalize();
            let casted = cast_ray(&origin, &direction, spheres, lights, 0);
            framebuffer.put_pixel(i, j, casted.to_rgb());
        }
    }

    framebuffer.save("./out.png")?;
    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut spheres = Vec::new();
    let mut lights = Vec::new();

    spheres.push(Sphere::new((-3.0, 0.0, -16.0).into(), 2.0, Material::IVORY));
    spheres.push(Sphere::new(
        (1.5, -0.5, -18.0).into(),
        3.0,
        Material::RED_RUBBER,
    ));
    spheres.push(Sphere::new(
        (-1.0, -1.5, -12.0).into(),
        2.0,
        Material::MIRROR,
    ));
    spheres.push(Sphere::new((7.0, 5.0, -18.0).into(), 4.0, Material::MIRROR));

    lights.push(Light::new((-20.0, 20.0, 20.0).into(), 1.5));
    lights.push(Light::new((30.0, 50.0, -25.0).into(), 1.8));
    lights.push(Light::new((30.0, 20.0, 30.0).into(), 1.7));

    render(&spheres, &lights)?;
    Ok(())
}
