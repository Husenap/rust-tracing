use crate::common::FP;

pub mod aabb;
pub mod color;
pub mod interval;
pub mod perlin;
pub mod ray;
pub mod vec3;

pub const PI: FP = std::f64::consts::PI as FP;

#[inline]
pub fn degrees_to_radians(degrees: FP) -> FP {
    degrees * PI / 180.0
}
