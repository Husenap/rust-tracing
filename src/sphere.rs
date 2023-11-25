use std::sync::Arc;

use crate::{
    aabb::AABB,
    common::{FP, PI},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: FP,
    material: Arc<dyn Material>,
    center_vec: Vec3,
    is_moving: bool,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Point3, radius: FP, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::splat(radius);
        Self {
            center,
            radius,
            material,
            center_vec: Vec3::ZERO,
            is_moving: false,
            bbox: AABB::new_from_points(center - rvec, center + rvec),
        }
    }
    pub fn with_target(self, target: Point3) -> Self {
        let rvec = Vec3::splat(self.radius);
        let box1 = AABB::new_from_points(self.center - rvec, self.center + rvec);
        let box2 = AABB::new_from_points(target - rvec, target + rvec);
        Self {
            center: self.center,
            radius: self.radius,
            material: self.material,
            center_vec: target - self.center,
            is_moving: true,
            bbox: AABB::new_from_aabbs(box1, box2),
        }
    }

    fn get_sphere_uv(&self, n: &Vec3) -> (FP, FP) {
        let theta = (-n.y).acos();
        let phi = (-n.z).atan2(n.x) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
    fn at(&self, time: FP) -> Point3 {
        self.center + self.center_vec * time
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            self.at(r.time)
        } else {
            self.center
        };
        let oc = r.origin - center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - center) / self.radius;
        let (u, v) = self.get_sphere_uv(&outward_normal);
        Some(HitRecord::new(p, self.material.as_ref(), root, r, outward_normal).with_uvs(u, v))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
