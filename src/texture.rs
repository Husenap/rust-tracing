use image::{DynamicImage, GenericImageView};

use crate::{
    common::FP,
    perlin::Perlin,
    vec3::{Color, Vec3},
};

pub trait Texture: Sync + Clone {
    fn value(&self, u: FP, v: FP, p: &Vec3) -> Color;
}

#[derive(Clone)]
pub struct SolidColor {
    color: Color,
}
impl SolidColor {
    pub fn new(red: FP, green: FP, blue: FP) -> SolidColor {
        Self {
            color: Color::new(red, green, blue),
        }
    }
}
impl From<Color> for SolidColor {
    fn from(value: Color) -> Self {
        Self { color: value }
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: FP, _v: FP, _p: &Vec3) -> Color {
        self.color
    }
}

#[derive(Clone)]
pub struct CheckerTexture<E: Texture, O: Texture> {
    inv_scale: FP,
    even: E,
    odd: O,
}
impl<E: Texture, O: Texture> CheckerTexture<E, O> {
    pub fn new(scale: FP, even: E, odd: O) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}
impl CheckerTexture<SolidColor, SolidColor> {
    pub fn new_from_colors(scale: FP, even: Color, odd: Color) -> Self {
        Self::new(scale, SolidColor::from(even), SolidColor::from(odd))
    }
}
impl<E: Texture, O: Texture> Texture for CheckerTexture<E, O> {
    fn value(&self, u: FP, v: FP, p: &Vec3) -> Color {
        let x = (self.inv_scale * p.x).floor() as i32;
        let y = (self.inv_scale * p.y).floor() as i32;
        let z = (self.inv_scale * p.z).floor() as i32;
        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    image: DynamicImage,
}
impl ImageTexture {
    pub fn new(path: &str) -> Self {
        Self {
            image: image::io::Reader::open(path).unwrap().decode().unwrap(),
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: FP, v: FP, _p: &Vec3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * (self.image.width() - 1) as FP) as u32;
        let j = (v * (self.image.height() - 1) as FP) as u32;
        let [r, g, b, _] = self.image.get_pixel(i, j).0;

        Color::new((r as FP) / 255.0, (g as FP) / 255.0, (b as FP) / 255.0)
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: FP,
}
impl NoiseTexture {
    pub fn new(scale: FP) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: FP, _v: FP, p: &Vec3) -> Color {
        Color::splat((self.scale * p.z + 10.0 * self.noise.turbulence(&p, 7)).sin() * 0.5 + 0.5)
    }
}
