use crate::{
    common::FP,
    math::{interval::Interval, vec3::Color},
};

#[inline]
fn linear_to_gamma(linear_component: FP) -> FP {
    linear_component.powf(1.0 / 2.2)
}
#[inline]
fn gamma_to_linear(gamma_component: FP) -> FP {
    gamma_component.powf(2.2)
}

pub fn color_to_rgb(rgb: &Color) -> [u8; 3] {
    let intensity = Interval::new(0.0, 0.999);
    [
        (256.0 * intensity.clamp(linear_to_gamma(rgb.x))) as u8,
        (256.0 * intensity.clamp(linear_to_gamma(rgb.y))) as u8,
        (256.0 * intensity.clamp(linear_to_gamma(rgb.z))) as u8,
    ]
}

pub fn rgb_to_color(r: u8, g: u8, b: u8) -> Color {
    Color::new(
        gamma_to_linear((r as FP) / 255.0),
        gamma_to_linear((g as FP) / 255.0),
        gamma_to_linear((b as FP) / 255.0),
    )
}
