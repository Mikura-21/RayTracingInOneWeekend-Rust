use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable, HittablePtr};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<HittablePtr>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::EMPTY,
        }
    }

    pub fn add(&mut self, object: HittablePtr) {
        self.bbox = Aabb::enclosing(self.bbox, object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
