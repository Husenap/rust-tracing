use camera::CameraSettings;
use common::FP;
use material::{Dielectric, Lambertian, Metal};
use rand::Rng;
use std::rc::Rc;
use vec3::Color;

use crate::{camera::Camera, hittable::HittableList, sphere::Sphere, vec3::Point3};

mod camera;
mod color;
mod common;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::splat(0.5)));
    world.add(Box::new(Sphere::new(
        Point3::DOWN * 1000.0,
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<FP>();

            let center = Point3::new(
                a as FP + 0.9 * rand::random::<FP>(),
                0.2,
                b as FP + 0.9 * rand::random::<FP>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Color::random() * Color::random())),
                    )));
                } else if choose_mat < 0.95 {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            Color::random_range(0.5, 1.0),
                            rand::thread_rng().gen_range(0.0..0.5),
                        )),
                    )));
                } else {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    let mut camera = Camera::new(CameraSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        samples_per_pixel: 500,
        max_depth: 50,
        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,
        ..Default::default()
    });

    camera.render(&world);

    Ok(())
}
