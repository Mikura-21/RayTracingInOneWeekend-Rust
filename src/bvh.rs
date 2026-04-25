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
        todo!()
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
