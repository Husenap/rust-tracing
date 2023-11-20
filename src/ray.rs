use crate::{
    common::FP,
    vec3::{Point3, Vec3},
};

#[derive(Default, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: FP,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn with_time(self, time: FP) -> Self {
        Self {
            origin: self.origin,
            direction: self.direction,
            time,
        }
    }

    pub fn at(&self, t: FP) -> Point3 {
        self.origin + t * self.direction
    }
}
