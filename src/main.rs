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
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use quad::Quad;
use rand::Rng;
use renderer::render;
use std::time::Instant;
use texture::NoiseTexture;
use vec3::{Color, Vec3};

mod aabb;
mod bvh;
mod camera;
mod color;
mod common;
mod hittable;
mod interval;
mod material;
mod perlin;
mod quad;
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

    /// Chooses scene index (0:random balls, 1:two spheres, 2:earth, 3:perlin spheres, 4:quads, 5:simple light)
    #[arg(short, long, default_value_t = 0)]
    scene: i32,

    /// Name of the output file that the render will end up in
    #[arg(short, long, default_value = "output")]
    output: String,
}

fn random_balls() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(SolidColor::from(Color::splat(0.5)));
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

    let camera = Camera::new(CameraSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 600,
        samples_per_pixel: 128,
        max_depth: 8,
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::ZERO,

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
        image_width: 1200,
        samples_per_pixel: 128,
        max_depth: 8,
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::ZERO,

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
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 20.0,
        look_from: Point3::new(12.0, 0.0, 0.0),
        look_at: Point3::ZERO,

        ..Default::default()
    });

    (world, camera)
}

fn two_perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let perlin_texture = NoiseTexture::new(4.0);

    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(perlin_texture.clone()),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(perlin_texture),
    ));

    let camera = Camera::new(CameraSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        samples_per_pixel: 128,
        max_depth: 8,
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::ZERO,

        ..Default::default()
    });

    (world, camera)
}

fn quads() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let left_red = Lambertian::new(SolidColor::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new(SolidColor::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(SolidColor::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new(SolidColor::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new(SolidColor::new(0.2, 0.8, 0.8));

    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::BACKWARD * 4.0,
        Vec3::UP * 4.0,
        left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::RIGHT * 4.0,
        Vec3::UP * 4.0,
        back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::FORWARD * 4.0,
        Vec3::UP * 4.0,
        right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::RIGHT * 4.0,
        Vec3::FORWARD * 4.0,
        upper_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::RIGHT * 4.0,
        Vec3::BACKWARD * 4.0,
        lower_teal,
    ));

    let camera = Camera::new(CameraSettings {
        aspect_ratio: 1.0,
        image_width: 1200,
        samples_per_pixel: 128,
        max_depth: 8,
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 80.0,
        look_from: Point3::FORWARD * 9.0,
        look_at: Point3::ZERO,

        ..Default::default()
    });

    (world, camera)
}

fn simple_light() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let perlin_texture = NoiseTexture::new(4.0);
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(perlin_texture.clone()),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(perlin_texture),
    ));

    let diffuse_light = SolidColor::new(4.0, 4.0, 4.0);
    world.add(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::RIGHT * 2.0,
        Vec3::UP * 2.0,
        DiffuseLight::new(diffuse_light.clone()),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        DiffuseLight::new(diffuse_light),
    ));

    let camera = Camera::new(CameraSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 600,
        samples_per_pixel: 1024,
        max_depth: 8,
        background: Color::ZERO,

        vfov: 20.0,
        look_from: Point3::new(26.0, 3.0, 6.0),
        look_at: Point3::UP * 2.0,

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
        3 => two_perlin_spheres(),
        4 => quads(),
        5 => simple_light(),
        _ => random_balls(),
    };

    let now = Instant::now();
    let bvh = BVHNode::new(&mut world);
    println!("Building BVH: {:.2?}", now.elapsed());

    if args.live {
        live_render(&camera, &bvh);
    } else {
        render(&camera, &bvh, args.output);
    }

    Ok(())
}
