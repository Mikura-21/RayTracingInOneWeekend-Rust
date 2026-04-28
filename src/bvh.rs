use std::cmp::Ordering;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable, HittablePtr};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BvhNode {
    left: HittablePtr,
    right: HittablePtr,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from_list(list: HittableList) -> Self {
        let mut objects = list.objects;
        Self::from_objects(&mut objects)
    }

    pub fn from_objects(objects: &mut [HittablePtr]) -> Self {
        let mut bbox = Aabb::EMPTY;
        for object in objects.iter() {
            bbox = Aabb::enclosing(bbox, object.bounding_box());
        }

        let axis = bbox.longest_axis();

        objects.sort_by(|a, b| box_compare(a, b, axis));

        let object_span = objects.len();

        let (left, right): (HittablePtr, HittablePtr) = match object_span {
            1 => {
                let obj = Arc::clone(&objects[0]);
                (Arc::clone(&obj), obj)
            }
            2 => (Arc::clone(&objects[0]), Arc::clone(&objects[1])),
            _ => {
                let mid = object_span / 2;
                (
                    Arc::new(Self::from_objects(&mut objects[..mid])),
                    Arc::new(Self::from_objects(&mut objects[mid..])),
                )
            }
        };

        Self { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);
        let max_for_right = match &hit_left {
            Some(hit) => hit.t,
            None => ray_t.max,
        };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, max_for_right));

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

fn box_compare(a: &HittablePtr, b: &HittablePtr, axis: usize) -> Ordering {
    let b_axis = a.bounding_box().axis_interval(axis).min;
    let a_axis = b.bounding_box().axis_interval(axis).min;

    a_axis.partial_cmp(&b_axis).unwrap_or(Ordering::Equal)
}
