use crate::{
    camera::Camera,
    color::{color_to_rgb, write_color},
    common::FP,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::Color,
};
use fltk::{app, prelude::*, window::Window};
use image::{codecs::png::CompressionType, ImageEncoder};
use indicatif::{ProgressBar, ProgressStyle};
use pixels::{Pixels, SurfaceTexture};
use rayon::prelude::*;
use std::io::Write;
use std::{fs::File, time::Instant};

pub fn render(camera: &Camera, world: &impl Hittable) {
    let height = camera.image_height;
    let width = camera.image_width;
    let spp = camera.samples_per_pixel;

    let bar = &Box::new(ProgressBar::new((width * height / 64) as u64));
    bar.set_prefix("🎥 Rendering");
    bar.set_style(
        ProgressStyle::with_template("{prefix:.bold} [{eta_precise}]▕{bar:64.}▏{percent}%")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    let now = Instant::now();
    let raw_pixels: Vec<Color> = (0..width * height)
        .into_par_iter()
        .map(|screen_pos| {
            let mut avg_color = Color::ZERO;
            let i = screen_pos % width;
            let j = screen_pos / width;

            for _ in 1..=spp {
                let r = camera.get_ray(i, j);
                let new_color = ray_color(&r, camera.max_depth, world);
                avg_color += new_color;
            }

            if screen_pos % 64 == 0 {
                bar.inc(1);
            }

            avg_color
        })
        .collect();
    bar.finish();
    println!("Render time: {:.2?}", now.elapsed());

    {
        let now = Instant::now();
        let pixels = raw_pixels
            .into_iter()
            .flat_map(|c| color_to_rgb(&(c / spp as FP)).to_vec())
            .collect::<Vec<u8>>();
        let output = File::create("output.png").unwrap();
        let encoder = image::codecs::png::PngEncoder::new_with_quality(
            output,
            CompressionType::Default,
            image::codecs::png::FilterType::Adaptive,
        );
        encoder
            .write_image(
                pixels.as_slice(),
                width as u32,
                height as u32,
                image::ColorType::Rgb8,
            )
            .expect("Should've encoded the image into a file.");
        println!("PNG encoding: {:.2?}", now.elapsed());
    }
}

pub fn live_render(camera: &Camera, world: &impl Hittable) {
    let height = camera.image_height;
    let width = camera.image_width;
    let spp = camera.samples_per_pixel;

    let app = app::App::default();
    let mut win = Window::default()
        .with_size(width as i32, height as i32)
        .with_label("rust-tracing");
    win.make_resizable(false);
    win.end();
    win.show();

    let mut pixels = {
        let pixel_width = win.pixel_w() as u32;
        let pixel_height = win.pixel_h() as u32;
        let surface_texture = SurfaceTexture::new(pixel_width, pixel_height, &win);
        Pixels::new(width as u32, height as u32, surface_texture).unwrap()
    };

    let mut raw_pixels: Vec<Color> = vec![Color::ZERO; width * height];
    let mut num_samples = 1;

    while app.wait() {
        win.set_label(format!("rust-tracing [{}x{}, spp:{}]", width, height, num_samples).as_str());

        // Draw the current frame
        if num_samples < spp {
            raw_pixels
                .par_iter_mut()
                .enumerate()
                .for_each(|(screen_pos, avg_color)| {
                    let i = screen_pos % width;
                    let j = screen_pos / width;

                    let r = camera.get_ray(i, j);
                    let new_color = ray_color(&r, camera.max_depth, world);
                    *avg_color += (new_color - *avg_color) / num_samples as FP;
                });
            num_samples += 1;

            let frame = pixels.frame_mut();
            frame
                .par_chunks_exact_mut(4)
                .enumerate()
                .for_each(|(i, pixel)| {
                    let [r, g, b] = color_to_rgb(&raw_pixels[i]);
                    let rgba = [r, g, b, 0xff];
                    pixel.copy_from_slice(&rgba);
                });
        }
        if let Err(err) = pixels.render() {
            println!("pixels.render: {}", err);
            app.quit();
        }

        app::flush();
        app::awake();
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
