use crate::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Default, Copy, Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }
    pub fn new_from_points(a: Point3, b: Point3) -> Self {
        Self::new(
            Interval::new(a.x.min(b.x), a.x.max(b.x)),
            Interval::new(a.y.min(b.y), a.y.max(b.y)),
            Interval::new(a.z.min(b.z), a.z.max(b.z)),
        )
    }
    pub fn new_from_aabbs(a: AABB, b: AABB) -> Self {
        Self::new(
            Interval::new_from_intervals(a.x, b.x),
            Interval::new_from_intervals(a.y, b.y),
            Interval::new_from_intervals(a.z, b.z),
        )
    }

    pub fn axis(&self, n: usize) -> &Interval {
        assert!(n <= 2);
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let orig = r.origin[a];

            let mut t0 = (self.axis(a).min - orig) * inv_d;
            let mut t1 = (self.axis(a).max - orig) * inv_d;

            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            }

            let t_min = t0.max(ray_t.min);
            let t_max = t1.min(ray_t.max);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
