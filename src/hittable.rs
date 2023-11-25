use std::sync::Arc;

use crate::{
    aabb::AABB,
    common::FP,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Material,
    pub t: FP,
    pub u: FP,
    pub v: FP,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point3, mat: &'a dyn Material, t: FP, r: &Ray, outward_normal: Vec3) -> Self {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        Self {
            p,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            mat,
            t,
            u: 0.0,
            v: 0.0,
            front_face,
        }
    }
    pub fn with_uvs(mut self, u: FP, v: FP) -> Self {
        self.u = u;
        self.v = v;
        self
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}
impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::new_from_aabbs(self.bbox, object.bounding_box());
        self.objects.push(Arc::clone(&object));
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
