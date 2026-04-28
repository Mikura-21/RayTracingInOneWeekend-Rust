use rand::{Rng, RngExt};

use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            randfloat: std::array::from_fn(|_| rng.random::<f64>()),
            perm_x: Self::generate_perm(rng),
            perm_y: Self::generate_perm(rng),
            perm_z: Self::generate_perm(rng),
        }
    }
    
    pub fn noise(&self, p: Point3) -> f64 {
        let i = ((4.0 * p.x) as i32 & 255) as usize;
        let j = ((4.0 * p.y) as i32 & 255) as usize;
        let k = ((4.0 * p.z) as i32 & 255) as usize;
        
        self.randfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
    
    fn generate_perm(rng: &mut impl Rng) -> [usize; POINT_COUNT] {
        let mut p = std::array::from_fn(|i| i);
        Self::permute(&mut p, rng);
        p
    }
    
    fn permute(p: &mut [usize; POINT_COUNT], rng: &mut impl Rng) {
        for i in (1..POINT_COUNT).rev() {
            let target = rng.random_range(0..=i);
            p.swap(i, target);
        }
    }
}
