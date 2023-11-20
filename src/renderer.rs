use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;

use crate::{
    camera::Camera, color::write_color, common::FP, hittable::Hittable, interval::Interval,
    ray::Ray, vec3::Color,
};

pub fn render(camera: &Camera, world: &impl Hittable) {
    let height = camera.image_height;
    let width = camera.image_width;
    let spp = camera.samples_per_pixel;

    let bar = &Box::new(ProgressBar::new((width * height / 64) as u64));
    bar.set_prefix("🎨  Rendering");
    bar.set_style(
        ProgressStyle::with_template("{prefix:.bold} [{eta_precise}]▕{bar:64.}▏{percent}%")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    let pixels: Vec<Color> = (0..width * height)
        .into_par_iter()
        .map(|screen_pos| {
            let mut pixel_color = Color::ZERO;
            let i = screen_pos % width;
            let j = screen_pos / width;

            for _ in 0..spp {
                let r = camera.get_ray(i, j);
                pixel_color += ray_color(&r, camera.max_depth, world);
            }

            if screen_pos % 64 == 0 {
                bar.inc(1);
            }

            pixel_color
        })
        .collect();

    bar.finish();

    let mut output = File::create("output.ppm").unwrap();
    writeln!(output, "P3").unwrap();
    writeln!(output, "{} {}", width, height).unwrap();
    writeln!(output, "255").unwrap();
    for pixel_color in pixels {
        write_color(&mut output, pixel_color, spp);
    }
}

fn ray_color(ray: &Ray, depth: i32, world: &impl Hittable) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }

    if let Some(hit) = world.hit(ray, &Interval::new(0.001, FP::INFINITY)) {
        if let Some((scattered, attenuation)) = hit.mat.scatter(ray, &hit) {
            return attenuation * ray_color(&scattered, depth - 1, world);
        }
        return Color::ZERO;
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::ONE + a * Color::new(0.5, 0.7, 1.0)
}
