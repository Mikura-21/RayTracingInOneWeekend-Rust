use std::sync::Arc;

use crate::color::Color;
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
