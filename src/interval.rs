use crate::common::FP;

#[derive(Default, Clone, Copy)]
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
    pub fn new_from_intervals(a: Interval, b: Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
    pub fn expand(self, delta: FP) -> Self {
        Self {
            min: self.min - delta * 0.5,
            max: self.max + delta * 0.5,
        }
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
