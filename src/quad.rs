use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
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
            self.mat.as_ref(),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub fn make_box(a: Point3, b: Point3, mat: MaterialPtr) -> HittableList {
    // Return the 3D box (six sides) that contains the two opposite vertices a & b.

    let mut sides = HittableList::new();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    // front
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        Arc::clone(&mat),
    )));
    // right
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        Arc::clone(&mat),
    )));
    // back
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        Arc::clone(&mat),
    )));
    // left
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        Arc::clone(&mat),
    )));
    // top
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        Arc::clone(&mat),
    )));
    // bottom
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        Arc::clone(&mat),
    )));

    sides
}
