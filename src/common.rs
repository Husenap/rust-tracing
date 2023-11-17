pub const PI: f32 = std::f32::consts::PI;
pub const INF: f32 = f32::INFINITY;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
