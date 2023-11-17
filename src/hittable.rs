use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Default, Copy, Clone, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut temp_rec = HitRecord {
            p: Point3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: false,
        };

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
