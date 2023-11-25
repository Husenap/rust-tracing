use crate::{
    aabb::AABB,
    common::{degrees_to_radians, FP},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

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

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}
impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}
impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(r.origin - self.offset, r.direction).with_time(r.time);

        if let Some(mut hit) = self.object.hit(&offset_r, ray_t) {
            hit.p += self.offset;
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: FP,
    cos_theta: FP,
    bbox: AABB,
}
impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: FP) -> Self {
        let theta = degrees_to_radians(angle);
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::INFINITY;
        let mut max = Point3::NEG_INFINITY;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as FP * bbox.x.max + (1.0 - i as FP) * bbox.x.min;
                    let y = j as FP * bbox.y.max + (1.0 - j as FP) * bbox.y.min;
                    let z = k as FP * bbox.z.max + (1.0 - k as FP) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::new_from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}
impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;

        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;

        let rotated_r = Ray::new(origin, direction).with_time(r.time);

        if let Some(mut hit) = self.object.hit(&rotated_r, ray_t) {
            let mut p = hit.p;
            p.x = self.cos_theta * hit.p.x + self.sin_theta * hit.p.z;
            p.z = -self.sin_theta * hit.p.x + self.cos_theta * hit.p.z;

            let mut normal = hit.normal;
            normal.x = self.cos_theta * hit.normal.x + self.sin_theta * hit.normal.z;
            normal.z = -self.sin_theta * hit.normal.x + self.cos_theta * hit.normal.z;

            hit.p = p;
            hit.normal = normal;

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
