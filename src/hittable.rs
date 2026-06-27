use std::sync::Arc;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: MaterialPtr,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        t: f64,
        u: f64,
        v: f64,
        r: &Ray,
        outward_normal: Vec3,
        mat: MaterialPtr,
    ) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            t,
            u,
            v,
            normal,
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

pub struct Translate {
    object: HittablePtr,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: HittablePtr, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_r = Ray::new(r.orig - self.offset, r.dir, r.time);

        // Determine wheter an intersection exists along the offset ray (and if so, where)
        let mut rec = self.object.hit(&offset_r, ray_t)?;

        // Move the intesection point forwards by the offset
        rec.p += self.offset;

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
