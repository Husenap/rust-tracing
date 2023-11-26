use crate::common::FP;
use core::fmt;
use rand::Rng;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: FP,
    pub y: FP,
    pub z: FP,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const INFINITY: Self = Self::splat(FP::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(FP::NEG_INFINITY);
    pub const RIGHT: Self = Self::new(1.0, 0.0, 0.0);
    pub const UP: Self = Self::new(0.0, 1.0, 0.0);
    pub const FORWARD: Self = Self::new(0.0, 0.0, 1.0);
    pub const LEFT: Self = Self::new(-1.0, 0.0, 0.0);
    pub const DOWN: Self = Self::new(0.0, -1.0, 0.0);
    pub const BACKWARD: Self = Self::new(0.0, 0.0, -1.0);

    #[inline(always)]
    pub const fn new(x: FP, y: FP, z: FP) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn splat(v: FP) -> Self {
        Self { x: v, y: v, z: v }
    }

    #[inline]
    pub fn random() -> Self {
        Self::new(rand::random(), rand::random(), rand::random())
    }
    #[inline]
    pub fn random_range(min: FP, max: FP) -> Self {
        Self::new(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }
    #[inline]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    #[inline]
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().normalize()
    }
    #[inline]
    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(&normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[inline]
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(
                rand::thread_rng().gen_range(-1.0..1.0),
                rand::thread_rng().gen_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    pub fn reflect(&self, n: &Self) -> Self {
        *self - 2.0 * self.dot(n) * *n
    }

    #[inline]
    pub fn refract(&self, n: &Self, etai_over_etat: FP) -> Self {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> FP {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn length_squared(&self) -> FP {
        self.dot(self)
    }

    pub fn near_zero(&self) -> bool {
        const EPS: FP = 1e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }

    #[inline]
    pub fn length(&self) -> FP {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn length_recip(&self) -> FP {
        self.length().recip()
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        *self * self.length_recip()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Default for Vec3 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for FP {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<FP> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, s: FP) -> Self::Output {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl MulAssign<FP> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, s: FP) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl Div<FP> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: FP) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<FP> for Vec3 {
    #[inline]
    fn div_assign(&mut self, s: FP) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

impl Index<usize> for Vec3 {
    type Output = FP;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index <= 2);
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index <= 2);
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
