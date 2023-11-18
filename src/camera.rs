use std::io::prelude::*;
use std::{fs::File, io::stdout};

use crate::{
    color::write_color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    output: File,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: i32) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as i32;
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let center = Point3::ZERO;
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let mut output = File::create("output.ppm").unwrap();
        writeln!(output, "P3").unwrap();
        writeln!(output, "{} {}", image_width, image_height).unwrap();
        writeln!(output, "255").unwrap();

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            output,
        }
    }

    pub fn render(&mut self, world: &impl Hittable) {
        for j in 0..self.image_height {
            print!("\rScanlines remaining: {}    ", self.image_height - j);
            stdout().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Color::ZERO;
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r, world);
                }
                write_color(&mut self.output, pixel_color, self.samples_per_pixel);
            }
        }
        println!("\rDone!                                        ");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand::random::<f32>();
        let py = -0.5 + rand::random::<f32>();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn ray_color(&self, ray: Ray, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(ray, Interval::new(0.0, f32::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::ONE);
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::ONE + a * Color::new(0.5, 0.7, 1.0)
    }
}