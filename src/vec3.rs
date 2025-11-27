use rand::Rng;
use rand::rngs::SmallRng;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// class vec3 {
//   public:
//     double e[3];
//
//     vec3() : e{0,0,0} {}
//     vec3(double e0, double e1, double e2) : e{e0, e1, e2} {}
//
//     double x() const { return e[0]; }
//     double y() const { return e[1]; }
//     double z() const { return e[2]; }
//
//     double operator[](int i) const { return e[i]; }
//     double& operator[](int i) { return e[i]; }
//
// };

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// // point3 is just an alias for vec3, but useful for geometric clarity in the code.
// using point3 = vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // class vec3 { public: double length_squared() const {}};
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // class vec3 { public: double length() const {}};
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    // inline double dot(const vec3& u, const vec3& v) {}
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // inline vec3 cross(const vec3& u, const vec3& v) {}
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // inline vec3 unit_vector(const vec3& v) {}
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random(rng: &mut SmallRng, min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: rng.random_range(min..max),
            y: rng.random_range(min..max),
            z: rng.random_range(min..max),
        }
    }

    pub fn random_unit_vector(rng: &mut SmallRng) -> Vec3 {
        loop {
            let p = Vec3::random(rng, -1.0, 1.0);
            let lensq = p.length_squared();
            if f64::MIN_POSITIVE < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_in_unit_disk(rng: &mut SmallRng) -> Vec3 {
        loop {
            let p = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(rng: &mut SmallRng, normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector(rng);
        // In the same hemisphere as the normal
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(self, n: Vec3) -> Self {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;

        r_out_perp + r_out_parallel
    }
}

// Overload operators
// inline std::ostream& operator<<(std::ostream& out, const vec3& v) {}
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// inline vec3 operator+(const vec3& u, const vec3& v) {}
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// class vec3 { public: vec3& operator+=(const vec3& v) {}};
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// inline vec3 operator-(const vec3& u, const vec3& v) {}
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// inline vec3 operator*(const vec3& u, const vec3& v) {}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// class vec3 { public: vec3 operator-() const {}};
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// inline vec3 operator*(const vec3& u, const vec3& v) {}
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// inline vec3 operator*(const vec3& v, double t) {}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

// inline vec3 operator*(double t, const vec3& v) {}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// class vec3 { public: vec3& operator*=(double t) {}};
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

// inline vec3 operator/(const vec3& v, double t) {}
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

// class vec3 { public: vec3& operator/=(double t){}};
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}
