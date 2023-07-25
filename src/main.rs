use glam::Vec3;
use image::{Rgb, RgbImage};

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    fn ray_intersect(&self, originates: &Vec3, direction: &Vec3) -> bool {
        // Produce a vector that is from the originating point
        // to the centre of the sphere
        let delta = self.center - originates.clone();

        // See if sphere is in the fornt of at behind of this direction.
        let projection = delta.dot(direction.clone());

        // Aright triangle formed by delta, projection and the orthogonal to
        // the projection. Check if that third distance is less than radius
        let orth_sq = delta.dot(delta) - projection * projection;
        let radius_sq = self.radius * self.radius;

        if orth_sq > radius_sq {
            return false;
        }

        // Now calculate the size of the intersected bit
        let half_segment = f32::sqrt(radius_sq - orth_sq);

        let first_intersection_distance = projection - half_segment;

        if first_intersection_distance < 0.0 {
            return false;
        }

        true
    }
}

fn cast_ray(origin: &Vec3, direction: &Vec3, sphere: &Sphere) -> Vec3 {
    if !sphere.ray_intersect(origin, direction) {
        return (0.2, 0.7, 0.8).into();
    }

    (0.4, 0.4, 0.3).into()
}

fn render() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let width: u32 = 1024; // px
    let height: u32 = 768; // px

    let mut framebuffer = RgbImage::new(width, height);

    for j in 0..height {
        for i in 0..width {
            let r = j as f32 / height as f32;
            let g = i as f32 / width as f32;
            let pixel = Rgb([
                (255.0 * f32::max(0.0, f32::min(r, 1.0))) as u8,
                (255.0 * f32::max(0.0, f32::min(g, 1.0))) as u8,
                0,
            ]);

            framebuffer.put_pixel(i, j, pixel);
        }
    }

    framebuffer.save("./out.png")?;
    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    render()?;
    Ok(())
}
