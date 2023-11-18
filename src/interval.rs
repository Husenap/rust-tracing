use crate::common::FP;

pub struct Interval {
    pub min: FP,
    pub max: FP,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: FP::INFINITY,
        max: FP::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        min: FP::NEG_INFINITY,
        max: FP::INFINITY,
    };

    pub fn new(min: FP, max: FP) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: FP) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: FP) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: FP) -> FP {
        x.clamp(self.min, self.max)
    }
}
