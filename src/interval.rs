use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn enclosing(a: Self, b: Self) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, displacement: f64) -> Self {
        Self {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, interval: Interval) -> Interval {
        interval + self
    }
}