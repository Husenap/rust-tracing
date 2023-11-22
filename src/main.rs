use crate::{
    camera::Camera,
    hittable::HittableList,
    renderer::live_render,
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, SolidColor},
    vec3::Point3,
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
mod texture;
mod vec3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable live rendering
    #[arg(short, long)]
    live: bool,

    /// Chooses scene index (0:random balls, 1:two spheres, 2:earth, 3:perlin spheres)
    #[arg(short, long, default_value_t = 0)]
    scene: i32,
}

fn random_balls() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    //let ground_material = Lambertian::new(Color::splat(0.5));
    let checker = CheckerTexture::new(
        0.64,
        SolidColor::new(0.2, 0.3, 0.1),
        CheckerTexture::new_from_colors(0.16, Color::new(0.1, 0.2, 0.3), Color::splat(0.9)),
    );
    let ground_material = Lambertian::new(checker.clone());
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
                    let color = SolidColor::from(Color::random() * Color::random());
                    world.add(
                        Sphere::new(center, 0.2, Lambertian::new(color))
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
        Lambertian::new(SolidColor::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    let earth_texture = ImageTexture::new("assets/monkes.jpg");
    world.add(Sphere::new(
        Point3::new(0.0, 3.0, 0.0),
        3.0,
        Lambertian::new(earth_texture),
    ));

    let camera = Camera::new(CameraSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 128,
        max_depth: 8,

        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,
        ..Default::default()
    });

    (world, camera)
}

fn two_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let checker =
        CheckerTexture::new_from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::splat(0.9));

    world.add(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(checker.clone()),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(checker),
    ));

    let camera = Camera::new(CameraSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 128,
        max_depth: 8,

        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),

        ..Default::default()
    });

    (world, camera)
}

fn earth() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let earth_texture = ImageTexture::new("assets/earth-large.jpg");

    world.add(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        Lambertian::new(earth_texture),
    ));

    let camera = Camera::new(CameraSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        samples_per_pixel: 128,
        max_depth: 8,

        vfov: 20.0,
        look_from: Point3::new(12.0, 0.0, 0.0),
        look_at: Point3::new(0.0, 0.0, 0.0),

        ..Default::default()
    });

    (world, camera)
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("Args: {:?}", args);

    let (mut world, camera) = match args.scene {
        0 => random_balls(),
        1 => two_spheres(),
        2 => earth(),
        _ => random_balls(),
    };

    let now = Instant::now();
    let bvh = BVHNode::new(&mut world);
    println!("Building BVH: {:.2?}", now.elapsed());

    if args.live {
        live_render(&camera, &bvh);
    } else {
        render(&camera, &bvh);
    }

    Ok(())
}
