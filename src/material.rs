use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use rand::rngs::SmallRng;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Color, Ray)>;
}
