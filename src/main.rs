use std::rc::Rc;

use material::{Dielectric, Lambertian, Metal};
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
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Dielectric::new(1.5));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);

    Ok(())
}
