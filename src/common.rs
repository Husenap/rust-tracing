pub type FP = f32;

pub const PI: FP = std::f64::consts::PI as FP;

#[inline]
pub fn degrees_to_radians(degrees: FP) -> FP {
    degrees * PI / 180.0
}
