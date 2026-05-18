use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: MaterialPtr,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: MaterialPtr) -> Self {
        let n = u.cross(v);
        let w = n / n.dot(n);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        Self {
            q: q,
            u: u,
            v: v,
            w: w,
            mat: mat,
            bbox: Self::compute_bounding_box(q, u, v),
            normal: normal,
            d: d,
        }
    }

    fn compute_bounding_box(q: Point3, u: Vec3, v: Vec3) -> Aabb {
        let bbox_diagonal1 = Aabb::from_points(q, q + u + v);
        let bbox_diagonal2 = Aabb::from_points(q + u, q + v);

        Aabb::enclosing(bbox_diagonal1, bbox_diagonal2)
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(r.dir);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(r.orig)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        Some(HitRecord::new(
            intersection,
            t,
            0.0,
            0.0,
            r,
            self.normal,
            Arc::clone(&self.mat),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
