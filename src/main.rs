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
use material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use quad::Quad;
use rand::Rng;
use renderer::render;
use std::{sync::Arc, time::Instant};
use texture::{NoiseTexture, Texture};
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

    /// Chooses scene index (0:random balls, 1:two spheres, 2:earth, 3:perlin spheres, 4:quads, 5:simple light, 6:cornell box)
    #[arg(short, long, default_value_t = 0)]
    scene: i32,

    /// Name of the output file that the render will end up in
    #[arg(short, long, default_value = "output")]
    output: String,
}

fn random_balls() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Arc::new(SolidColor::from(Color::splat(
        0.5,
    )))));
    world.add(Arc::new(Sphere::new(
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
                    let color = Arc::new(SolidColor::from(Color::random() * Color::random()));
                    world.add(Arc::new(
                        Sphere::new(center, 0.2, Arc::new(Lambertian::new(color)))
                            .with_target(center + Vec3::UP * rand::random::<FP>() * 0.5),
                    ));
                } else if choose_mat < 0.95 {
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Color::random_range(0.5, 1.0),
                            rand::thread_rng().gen_range(0.0..0.5),
                        )),
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.4, 0.2, 0.1)))),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

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

    let checker: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(
        CheckerTexture::new_from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::splat(0.9)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::clone(&checker),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker,
    )));

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

    let earth_texture = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
        "assets/earth-large.jpg",
    ))));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_texture,
    )));

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

    let perlin_texture: Arc<dyn Material> =
        Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(4.0))));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&perlin_texture),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_texture,
    )));

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

    let left_red = Arc::new(Lambertian::new(Arc::new(SolidColor::new(1.0, 0.2, 0.2))));
    let back_green = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.2, 1.0, 0.2))));
    let right_blue = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.2, 0.2, 1.0))));
    let upper_orange = Arc::new(Lambertian::new(Arc::new(SolidColor::new(1.0, 0.5, 0.0))));
    let lower_teal = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.2, 0.8, 0.8))));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::BACKWARD * 4.0,
        Vec3::UP * 4.0,
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::RIGHT * 4.0,
        Vec3::UP * 4.0,
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::FORWARD * 4.0,
        Vec3::UP * 4.0,
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::RIGHT * 4.0,
        Vec3::FORWARD * 4.0,
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::RIGHT * 4.0,
        Vec3::BACKWARD * 4.0,
        lower_teal,
    )));

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

    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Arc::clone(&perlin_texture))),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));

    let diffuse_light: Arc<dyn Material> =
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(4.0, 4.0, 4.0))));

    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::RIGHT * 2.0,
        Vec3::UP * 2.0,
        Arc::clone(&diffuse_light),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light,
    )));

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

fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.65, 0.05, 0.05))));
    let white: Arc<dyn Material> =
        Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.73, 0.73, 0.73))));
    let green = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.12, 0.45, 0.15))));
    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(
        15.0, 15.0, 15.0,
    ))));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 555.0),
        Vec3::UP * 555.0,
        Vec3::BACKWARD * 555.0,
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::ZERO,
        Vec3::UP * 555.0,
        Vec3::FORWARD * 555.0,
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::LEFT * 130.0,
        Vec3::BACKWARD * 105.0,
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::FORWARD * 555.0,
        Vec3::RIGHT * 555.0,
        Vec3::BACKWARD * 555.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Point3::ONE * 555.0,
        Vec3::LEFT * 555.0,
        Vec3::BACKWARD * 555.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 555.0),
        Vec3::LEFT * 555.0,
        Vec3::UP * 555.0,
        Arc::clone(&white),
    )));

    world.add(Quad::cube(
        &Point3::new(130.0, 0.0, 65.0),
        &Point3::new(295.0, 165.0, 230.0),
        Arc::clone(&white),
    ));
    world.add(Quad::cube(
        &Point3::new(265.0, 0.0, 295.0),
        &Point3::new(430.0, 330.0, 460.0),
        white,
    ));

    let camera = Camera::new(CameraSettings {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 1024,
        max_depth: 8,
        background: Color::ZERO,

        vfov: 40.0,
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),

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
        6 => cornell_box(),
        _ => random_balls(),
    };

    let now = Instant::now();
    let bvh = BVHNode::new(&mut world);
    println!("Building BVH: {:.2?}", now.elapsed());

    if args.live {
        live_render(Arc::new(camera), Arc::new(bvh));
    } else {
        render(Arc::new(camera), Arc::new(bvh), args.output);
    }

    Ok(())
}
