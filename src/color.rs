use crate::{common::FP, interval::Interval, vec3::Color};

#[inline]
fn linear_to_gamma(linear_component: FP) -> FP {
    linear_component.powf(0.4545)
}

pub fn color_to_rgb(rgb: &Color) -> [u8; 3] {
    let intensity = Interval::new(0.0, 0.999);
    [
        (256.0 * intensity.clamp(linear_to_gamma(rgb.x))) as u8,
        (256.0 * intensity.clamp(linear_to_gamma(rgb.y))) as u8,
        (256.0 * intensity.clamp(linear_to_gamma(rgb.z))) as u8,
    ]
}
