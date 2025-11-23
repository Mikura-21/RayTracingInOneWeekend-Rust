use std::io::{self, Write};

mod color;
mod ray;
mod vec3;

use color::Color;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let pixel_color = Color::new(r, g, b);

            color::write_color(&pixel_color);
        }
    }
    eprintln!("\nDone.                 ");
}
