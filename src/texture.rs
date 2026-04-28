use std::sync::Arc;

use crate::color::Color;
use crate::rtw_image::RtwImage;
use crate::vec3::Point3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub type TexturePtr = Arc<dyn Texture>;

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: TexturePtr,
    odd: TexturePtr,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: TexturePtr, odd: TexturePtr) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_int = (self.inv_scale * p.x).floor() as i64;
        let y_int = (self.inv_scale * p.y).floor() as i64;
        let z_int = (self.inv_scale * p.z).floor() as i64;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        // If we hae no texture data, then return solid cyan as a debugging aid.
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;

        let r = color_scale * pixel[0] as f64;
        let g = color_scale * pixel[1] as f64;
        let b = color_scale * pixel[2] as f64;
        // Convert texture color from gamma space to linear space.
        Color::new(r * r, g * g, b * b)
    }
}
