use crate::{camera::Camera, hittable::HittableList, sphere::Sphere, vec3::Point3};

mod camera;
mod color;
mod common;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));

    let mut camera = Camera::new(16.0 / 9.0, 400, 10);
    camera.render(&world);

    Ok(())
}
