use image::{Rgb, RgbImage};

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
