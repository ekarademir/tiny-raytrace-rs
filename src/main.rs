use glam::Vec3;
use image::{Rgb, RgbImage};

#[derive(Copy, Clone)]
struct Light {
    position: Vec3,
    intensity: f32,
}

impl Default for Light {
    fn default() -> Self {
        Light {
            position: (-20.0, 20.0, 20.0).into(),
            intensity: 1.5,
        }
    }
}

#[derive(Copy, Clone)]
struct Material {
    diffuse_colour: Vec3,
}

impl Material {
    fn new(diffuse_colour: Vec3) -> Self {
        Material { diffuse_colour }
    }

    fn ivory() -> Self {
        Self::new((0.4, 0.4, 0.3).into())
    }

    fn red_rubber() -> Self {
        Self::new((0.3, 0.1, 0.1).into())
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            diffuse_colour: (0.2, 0.7, 0.8).into(),
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

        let first_intersection_distance = projection - half_segment;

        if first_intersection_distance < 0.0 {
            return Some(delta.dot(*direction) + half_segment);
        }

        Some(delta.dot(*direction) - half_segment)
    }
}

fn cast_ray(origin: &Vec3, direction: &Vec3, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Vec3 {
    match scene_intersect(origin, direction, spheres) {
        Some((material, point, normal)) => {
            let mut diffuse_light_intensity: f32 = 0.0;

            for light in lights {
                let light_dir = (light.position - point).normalize();
                diffuse_light_intensity += light.intensity * f32::max(0.0, light_dir.dot(normal));
            }

            return material.diffuse_colour * diffuse_light_intensity;
        }
        None => Material::default().diffuse_colour,
    }
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
            normal = (hit.clone() - sphere.center).normalize();
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
            let casted = cast_ray(&origin, &direction, spheres, lights);

            let pixel = Rgb([
                (255.0 * f32::max(0.0, f32::min(casted.x, 1.0))) as u8,
                (255.0 * f32::max(0.0, f32::min(casted.y, 1.0))) as u8,
                (255.0 * f32::max(0.0, f32::min(casted.z, 1.0))) as u8,
            ]);

            framebuffer.put_pixel(i, j, pixel);
        }
    }

    framebuffer.save("./out.png")?;
    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut spheres = Vec::new();
    let mut lights = Vec::new();

    spheres.push(Sphere::new(
        (-3.0, 0.0, -16.0).into(),
        2.0,
        Material::ivory(),
    ));
    spheres.push(Sphere::new(
        (-1.0, -1.5, -12.0).into(),
        2.0,
        Material::red_rubber(),
    ));
    spheres.push(Sphere::new(
        (1.5, -0.5, -18.0).into(),
        3.0,
        Material::red_rubber(),
    ));
    spheres.push(Sphere::new(
        (7.0, 5.0, -18.0).into(),
        4.0,
        Material::ivory(),
    ));

    lights.push(Light::default());

    render(&spheres, &lights)?;
    Ok(())
}
