use std::sync::Arc;

use crate::aabb::Aabb;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable::HittablePtr;
use crate::interval::Interval;
use crate::material::Isotropic;
use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::texture::TexturePtr;
use crate::vec3::Vec3;

pub struct ConstantMedium {
    boundary: HittablePtr,
    neg_inv_density: f64,
    phase_function: MaterialPtr,
}

impl ConstantMedium {
    pub fn from_texture(boundary: HittablePtr, density: f64, tex: TexturePtr) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_texture(tex)),
        }
    }

    pub fn from_color(boundary: HittablePtr, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut rec1 = self
            .boundary
            .hit(r, Interval::new(f64::NEG_INFINITY, f64::INFINITY))?;

        let mut rec2 = self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY))?;

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        let normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        let front_face = true; // also arbitrary
        let mat = self.phase_function.as_ref();

        Some(HitRecord::new(p, t, 0.0, 0.0, r, normal, mat))
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
