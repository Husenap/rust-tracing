use std::io::prelude::*;
use std::{fs::File, io::stdout};

use crate::common::{degrees_to_radians, FP};
use crate::{
    color::write_color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

pub struct CameraSettings {
    pub aspect_ratio: FP,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: FP,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: FP,
    pub focus_dist: FP,
}
impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            vfov: 90.0,
            look_from: Point3::ZERO,
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::UP,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}
pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: FP,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    output: File,
}

impl Camera {
    pub fn new(settings: CameraSettings) -> Self {
        let CameraSettings {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_dist,
        } = settings;

        let image_height = (image_width as FP / aspect_ratio) as i32;

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as FP / image_height as FP);

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let center = look_from;
        let pixel_delta_u = viewport_u / image_width as FP;
        let pixel_delta_v = viewport_v / image_height as FP;

        let viewport_upper_left = center - focus_dist * w - viewport_u * 0.5 - viewport_v * 0.5;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let mut output = File::create("output.ppm").unwrap();
        writeln!(output, "P3").unwrap();
        writeln!(output, "{} {}", image_width, image_height).unwrap();
        writeln!(output, "255").unwrap();

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
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
                    pixel_color += self.ray_color(r, self.max_depth, world);
                }
                write_color(&mut self.output, pixel_color, self.samples_per_pixel);
            }
        }
        println!("\rDone!                                        ");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as FP * self.pixel_delta_u) + (j as FP * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand::random::<FP>();
        let py = -0.5 + rand::random::<FP>();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn ray_color(&self, ray: Ray, depth: i32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::ZERO;
        }

        let mut rec = HitRecord::default();
        if world.hit(ray, Interval::new(0.001, FP::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            if let Some(mat) = &rec.mat {
                if mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * self.ray_color(scattered, depth - 1, world);
                }
            }

            return Color::ZERO;
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::ONE + a * Color::new(0.5, 0.7, 1.0)
    }
}
