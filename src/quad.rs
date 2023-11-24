use crate::{
    aabb::AABB,
    common::FP,
    hittable::{HitRecord, Hittable},
    material::Material,
    vec3::{Point3, Vec3},
};

pub struct Quad<M: Material> {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: M,
    bbox: AABB,
    d: FP,
    normal: Vec3,
}

impl<M: Material> Quad<M> {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: M) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);
        let w = n / n.length_squared();

        Self {
            q,
            u,
            v,
            w,
            mat,
            bbox: Self::create_bounding_box(&q, &u, &v),
            d,
            normal,
        }
    }

    fn create_bounding_box(q: &Vec3, u: &Vec3, v: &Vec3) -> AABB {
        AABB::new_from_points(*q, *q + *u + *v).pad()
    }
}

impl<M: Material> Hittable for Quad<M> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        let denom = self.normal.dot(&r.direction);

        /*
         * back-face culling for quads?
        if denom > 1e-8 {
            return None;
        }
        */
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(&r.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let planar_hit_point = intersection - self.q;
        let alpha = self.w.dot(&planar_hit_point.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_point));

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }

        Some(HitRecord::new(intersection, &self.mat, t, r, self.normal).with_uvs(alpha, beta))
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.bbox
    }
}
