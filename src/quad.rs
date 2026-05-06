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
    mat: MaterialPtr,
    bbox: Aabb,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: MaterialPtr) -> Self {
        Self {
            q: q,
            u: u,
            v: v,
            mat: mat,
            bbox: Self::compute_bounding_box(q, u, v),
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
        None // To be implemented
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
