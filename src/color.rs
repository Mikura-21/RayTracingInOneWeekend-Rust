use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    println!("{} {} {}", ir, ig, ib);
}
