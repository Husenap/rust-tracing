use std::sync::Arc;

use crate::{
    common::FP,
    hittable::{HitRecord, Hittable},
    material::{Isotropic, Material},
    math::{aabb::AABB, interval::Interval, ray::Ray, vec3::Color},
    texture::{SolidColor, Texture},
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: FP,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: FP, albedo: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(albedo)),
        }
    }
    pub fn new_from_color(boundary: Arc<dyn Hittable>, density: FP, albedo: Color) -> Self {
        Self::new(boundary, density, Arc::new(SolidColor::from(albedo)))
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(mut hit1) = self.boundary.hit(r, &Interval::UNIVERSE) {
            if let Some(mut hit2) = self
                .boundary
                .hit(r, &Interval::new(hit1.t + 0.0001, FP::INFINITY))
            {
                hit1.t = hit1.t.max(ray_t.min);
                hit2.t = hit2.t.min(ray_t.max);

                if hit1.t < hit2.t {
                    hit1.t = hit1.t.max(0.0);

                    let ray_length = r.direction.length();
                    let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                    let hit_distance = self.neg_inv_density * rand::random::<FP>().ln();

                    if hit_distance <= distance_inside_boundary {
                        let t = hit1.t + hit_distance / ray_length;
                        Some(HitRecord::new(
                            r.at(t),
                            self.phase_function.as_ref(),
                            t,
                            r,
                            r.direction,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
