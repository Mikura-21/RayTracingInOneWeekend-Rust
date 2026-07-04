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

pub struct RotateY {
    object: HittablePtr,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: HittablePtr, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    min.x = min.x.min(tester.x);
                    min.y = min.y.min(tester.y);
                    min.z = min.z.min(tester.z);

                    max.x = max.x.max(tester.x);
                    max.y = max.y.max(tester.y);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: Aabb::from_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Transform the ray from world space to object space.
        let origin = Point3::new(
            self.cos_theta * r.orig.x - self.sin_theta * r.orig.z,
            r.orig.y,
            self.sin_theta * r.orig.x + self.cos_theta * r.orig.z,
        );
        let direction = Vec3::new(
            self.cos_theta * r.dir.x - self.sin_theta * r.dir.z,
            r.dir.y,
            self.sin_theta * r.dir.x + self.cos_theta * r.dir.z,
        );
        let rotated_r = Ray::new(origin, direction, r.time);

        // Determine wheher an intersection exists in object space (and if so, where).
        let mut rec = self.object.hit(&rotated_r, ray_t)?;

        // Transform the intersection from object space back to world space.
        rec.p = Point3::new(
            self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
            rec.p.y,
            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
        );
        rec.normal = Vec3::new(
            self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
            rec.normal.y,
            -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
        );

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}