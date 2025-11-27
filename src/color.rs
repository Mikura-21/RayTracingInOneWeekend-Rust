use crate::vec3::Vec3;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.max(0.0).sqrt()
}

pub fn write_color(pixel_color: &Color) {
    // Apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;

    println!("{} {} {}", ir, ig, ib);
}
