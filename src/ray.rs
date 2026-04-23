use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            time: time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

// #ifndef RAY_H
// #define RAY_H
//
// #include "vec3.h"
//
// class ray {
//   public:
//     ray() {}
//
//     ray(const point3& origin, const vec3& direction) : orig(origin), dir(direction) {}
//
//     const point3& origin() const  { return orig; }
//     const vec3& direction() const { return dir; }
//
//     point3 at(double t) const {
//         return orig + t*dir;
//     }
//
//   private:
//     point3 orig;
//     vec3 dir;
// };
//
// #endif
