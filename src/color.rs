use crate::{interval::Interval, vec3::Color};

pub fn write_color(output: &mut impl std::io::Write, pixel_color: Color, samples_per_pixel: i32) {
    let rgb = pixel_color / samples_per_pixel as f32;

    let intensity = Interval::new(0.0, 0.999);

    writeln!(
        *output,
        "{} {} {}",
        (256.0 * intensity.clamp(rgb.x)) as i32,
        (256.0 * intensity.clamp(rgb.y)) as i32,
        (256.0 * intensity.clamp(rgb.z)) as i32
    )
    .expect("Should write to file");
}
