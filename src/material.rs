use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use rand::rngs::SmallRng;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Color::random_unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}
