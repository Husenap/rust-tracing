use crate::{
    camera::Camera, hittable::HittableList, renderer::live_render, sphere::Sphere, vec3::Point3,
};
use bvh::BVHNode;
use camera::CameraSettings;
use clap::Parser;
use common::FP;
use material::{Dielectric, Lambertian, Metal};
use rand::Rng;
use renderer::render;
use std::time::Instant;
use vec3::{Color, Vec3};

mod aabb;
mod bvh;
mod camera;
mod color;
mod common;
mod hittable;
mod interval;
mod material;
mod ray;
mod renderer;
mod sphere;
mod vec3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable live rendering
    #[arg(short, long)]
    live: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("Args: {:?}", args);

    let mut world = HittableList::default();

    let ground_material = Lambertian::new(Color::splat(0.5));
    world.add(Sphere::new(Point3::DOWN * 1000.0, 1000.0, ground_material));

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
                    world.add(
                        Sphere::new(
                            center,
                            0.2,
                            Lambertian::new(Color::random() * Color::random()),
                        )
                        .with_target(center + Vec3::UP * rand::random::<FP>() * 0.5),
                    );
                } else if choose_mat < 0.95 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Color::random_range(0.5, 1.0),
                            rand::thread_rng().gen_range(0.0..0.5),
                        ),
                    ));
                } else {
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    let now = Instant::now();
    let bvh = BVHNode::new(&mut world);
    println!("Building BVH: {:.2?}", now.elapsed());

    let camera = Camera::new(CameraSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 64,
        max_depth: 16,
        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,
        ..Default::default()
    });

    if args.live {
        live_render(&camera, &bvh);
    } else {
        let now = Instant::now();
        render(&camera, &bvh);
        println!("Render time: {:.2?}", now.elapsed());
    }

    Ok(())
}
