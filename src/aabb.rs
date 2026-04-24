use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.
        Self {
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        }
    }

    pub fn enclosing(box0: Aabb, box1: Aabb) -> Self {
        Self {
            x: Interval::enclosing(box0.x, box1.x),
            y: Interval::enclosing(box0.y, box1.y),
            z: Interval::enclosing(box0.z, box1.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("axis must be 0, 1, or 2, got {n}"),
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.orig;
        let ray_dir = r.dir;

        for axis in 0..3 {
            let ax = Self::axis_interval(&self, axis);
            let adinv = 1.0 / ray_dir.axis(axis);

            let t0 = (ax.min - ray_orig.axis(axis)) * adinv;
            let t1 = (ax.max - ray_orig.axis(axis)) * adinv;

            ray_t.min = ray_t.min.max(t0.min(t1));
            ray_t.max = ray_t.max.min(t0.max(t1));

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
