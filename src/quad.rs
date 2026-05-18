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

    fn is_interior(a: f64, b: f64) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        unit_interval.contains(a) && unit_interval.contains(b)
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

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !Self::is_interior(alpha, beta) {
            return None;
        }

        // Ray hits the 2D shape; return Some(HitRecord)
        Some(HitRecord::new(
            intersection,
            t,
            alpha,
            beta,
            r,
            self.normal,
            Arc::clone(&self.mat),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
