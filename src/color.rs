use crate::{common::FP, interval::Interval, vec3::Color};

#[inline]
fn linear_to_gamma(linear_component: FP) -> FP {
    linear_component.powf(0.4545)
}

pub fn write_color(output: &mut impl std::io::Write, pixel_color: Color, samples_per_pixel: i32) {
    let rgb = pixel_color / samples_per_pixel as FP;

    let intensity = Interval::new(0.0, 0.999);

    writeln!(
        *output,
        "{} {} {}",
        (256.0 * intensity.clamp(linear_to_gamma(rgb.x))) as i32,
        (256.0 * intensity.clamp(linear_to_gamma(rgb.y))) as i32,
        (256.0 * intensity.clamp(linear_to_gamma(rgb.z))) as i32
    )
    .expect("Should write to file");
}
