use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Ray, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }

    // Stationary Sphere
    pub fn new_stationary(static_center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0), 0.0),
            radius: radius.max(0.0),
            mat: mat,
        }
    }

    // Moving Sphere
    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        Sphere {
            center: Ray::new(center1, center2 - center1, 0.0),
            radius: radius.max(0.0),
            mat: mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time);
        let oc: Vec3 = current_center - r.orig;
        let a = r.dir.length_squared();
        let h = r.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - current_center) / self.radius;
        Some(HitRecord::new(p, t, r, outward_normal, self.mat.clone()))
    }
}
