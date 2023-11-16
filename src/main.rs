use std::fs::File;
use std::io::prelude::*;

use vec3::Color;

mod vec3;

struct RayTracer {
    output: File,
}

impl RayTracer {
    pub fn new(output: File) -> Self {
        Self { output }
    }
}

fn write_color(output: &mut impl Write, pixel_color: Color) {
    writeln!(
        *output,
        "{} {} {}",
        (255.999 * pixel_color.x) as i32,
        (255.999 * pixel_color.y) as i32,
        (255.999 * pixel_color.z) as i32
    )
    .expect("Should write to file");
}

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut output = File::create("output.ppm")?;

    writeln!(output, "P3")?;
    writeln!(output, "{} {}", image_width, image_height)?;
    writeln!(output, "255")?;

    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.0,
            );

            write_color(&mut output, pixel_color);
        }
    }
    println!("\rDone!                                        ");

    Ok(())
}
