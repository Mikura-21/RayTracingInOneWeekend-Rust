use std::sync::Arc;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: MaterialPtr,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, r: &Ray, outward_normal: Vec3, mat: MaterialPtr) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }
}

pub type HittablePtr = Arc<dyn Hittable + Send + Sync>;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}
