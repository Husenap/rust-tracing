use std::fs::File;
use std::io::prelude::*;

use ray::Ray;
use vec3::Color;

use crate::vec3::{Point3, Vec3};

mod ray;
mod vec3;

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

fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * (n + Vec3::ONE);
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::ONE + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::ZERO;

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut output = File::create("output.ppm")?;

    writeln!(output, "P3")?;
    writeln!(output, "{} {}", image_width, image_height)?;
    writeln!(output, "255")?;

    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);

            write_color(&mut output, pixel_color);
        }
    }
    println!("\rDone!                                        ");

    Ok(())
}
