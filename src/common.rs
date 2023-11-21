pub type FP = f64;

pub const PI: FP = std::f64::consts::PI as FP;

#[inline]
pub fn degrees_to_radians(degrees: FP) -> FP {
    degrees * PI / 180.0
}
