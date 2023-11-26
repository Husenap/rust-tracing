use crate::common::FP;
use crate::math::degrees_to_radians;
use crate::math::ray::Ray;
use crate::math::vec3::{Color, Point3, Vec3};

pub struct CameraSettings {
    pub aspect_ratio: FP,
    pub image_width: usize,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: FP,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: FP,
    pub focus_dist: FP,
    pub background: Color,
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
            background: Color::ZERO,
        }
    }
}
pub struct Camera {
    pub image_width: usize,
    pub image_height: usize,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub background: Color,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: FP,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
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
            background,
        } = settings;

        let image_height = (image_width as FP / aspect_ratio) as usize;

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as FP / image_height as FP);

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

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

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            background,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as FP * self.pixel_delta_u) + (j as FP * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rand::random();

        Ray::new(ray_origin, ray_direction).with_time(ray_time)
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
}
